use std::{error::Error, mem::ManuallyDrop, pin::Pin};

pub struct MagicalDeserializer<'r> {
    /// Don't R/W from this address until dropping the "dependant",
    /// which uniquely borrows the addressed value.
    boxed_dependency: *mut (dyn crate::util::Something + 'r),
    /// Should be dropped first. Should not be taken out of the field.
    erased_dependant: ManuallyDrop<Pin<Box<dyn erased_serde::Deserializer<'r> + 'r>>>,
}

impl<'r> MagicalDeserializer<'r> {
    pub fn from_direct_impl<T>(typed_deserializer: T) -> Self
    where
        T: serde::Deserializer<'r> + 'r,
    {
        Self {
            boxed_dependency: Box::leak(Box::new(())),
            erased_dependant: ManuallyDrop::new(Box::pin(<dyn erased_serde::Deserializer>::erase(
                typed_deserializer,
            ))),
        }
    }

    pub fn new<T>(typed_deserializer: T) -> Self
    where
        T: 'r,
        &'r mut T: serde::Deserializer<'r>,
    {
        let boxed_dependency: *mut T = Box::leak(Box::new(typed_deserializer));
        Self {
            boxed_dependency,
            erased_dependant: ManuallyDrop::new(Box::pin(<dyn erased_serde::Deserializer>::erase(
                unsafe { &mut *boxed_dependency },
            ))),
        }
    }

    pub fn deserialize<'o, O: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<O, impl Error + 'static> {
        erased_serde::deserialize(unsafe {
            // We trust the called function to not take the value out of the field.
            self.erased_dependant.as_mut().get_unchecked_mut()
        })
    }
}
impl<'r> Drop for MagicalDeserializer<'r> {
    fn drop(&mut self) {
        unsafe {
            drop(ManuallyDrop::take(&mut self.erased_dependant));
            drop(Box::from_raw(self.boxed_dependency));
        }
    }
}

#[cfg(test)]
mod test {
    cfg_if::cfg_if! {
        if #[cfg(feature="json")] {
            use std::sync::LazyLock;
            static JSON_OBJ: LazyLock<&str> = LazyLock::new(|| "{\"zero\":0,\"k\":42}");
        }
    }

    #[cfg(feature = "json")]
    #[test]
    fn deserializes_int_from_json() {
        let json_data = b"3";
        let mut source = &json_data[..];
        let typed_deserializer = serde_json::Deserializer::from_reader(&mut source);
        let mut magical_deserializer = crate::MagicalDeserializer::new(typed_deserializer);
        let result: i32 = magical_deserializer.deserialize().unwrap();
        assert_eq!(result, 3);
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn deserializes_int_from_yaml() {
        let yaml_data = b"2\n";
        let mut source = &yaml_data[..];
        let typed_deserializer = serde_yaml::Deserializer::from_reader(&mut source);
        let mut magical_deserializer =
            crate::MagicalDeserializer::from_direct_impl(typed_deserializer);
        let result: i32 = magical_deserializer.deserialize().unwrap();
        assert_eq!(result, 2);
    }

    #[cfg(feature = "json")]
    #[test]
    fn deserializes_static_obj_from_json() {
        let mut source = JSON_OBJ.as_bytes();
        let typed_deserializer = serde_json::Deserializer::from_reader(&mut source);
        let mut magical_deserializer = crate::MagicalDeserializer::new(typed_deserializer);
        let result: serde_json::Value = magical_deserializer.deserialize().unwrap();
        let expected: serde_json::Value = serde_json::from_str(&JSON_OBJ).unwrap();
        assert_eq!(result, expected);
    }
}

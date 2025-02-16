use std::{error::Error, mem::ManuallyDrop, pin::Pin};

pub struct MagicalSerializer<'w> {
    /// Don't R/W from this address untill dropping the "dependant",
    /// which uniquely borrows the addressed value.
    boxed_dependency: *mut (dyn crate::util::Something + 'w),
    /// Should be dropped first. Should not be taken out of the field.
    erased_dependant:
        ManuallyDrop<Pin<Box<dyn erased_serde::Serializer + 'w>>>,
}
impl<'w> MagicalSerializer<'w> {
    pub fn new<T>(typed_serializer: T) -> Self
    where
        T: 'w,
        for<'any> &'any mut T: serde::Serializer,
    {
        let boxed_dependency: *mut T =
            Box::leak(Box::new(typed_serializer));
        Self {
            boxed_dependency,
            erased_dependant: ManuallyDrop::new(Box::pin(
                <dyn erased_serde::Serializer>::erase(unsafe {
                    &mut *boxed_dependency
                }),
            )),
        }
    }
    pub fn serialize<'o, O: serde::Serialize + ?Sized>(
        &mut self,
        serializable: &'o O,
    ) -> Result<(), impl Error + 'static> {
        erased_serde::Serialize::erased_serialize(
            serializable,
            unsafe {
                // We trust the called function to not take the value out of the field.
                self.erased_dependant.as_mut().get_unchecked_mut()
            },
        )
    }
}
impl<'w> Drop for MagicalSerializer<'w> {
    fn drop(&mut self) {
        unsafe {
            drop(ManuallyDrop::take(&mut self.erased_dependant));
            std::ptr::drop_in_place(self.boxed_dependency);
        }
    }
}

#[cfg(test)]
mod test {
    cfg_if::cfg_if! {
        if #[cfg(any(feature="json", feature="yaml"))] {
            use std::{collections::HashMap, sync::LazyLock};
            static SERIALIZABLE_STATIC_OBJ: LazyLock<HashMap<&str, u32>> =
                LazyLock::new(|| [("k", 42), ("zero", 0)].into());
        }
    }

    #[cfg(feature = "json")]
    #[test]
    fn serializes_int_to_json() {
        let mut sink = Vec::new();
        {
            let typed_serializer =
                serde_json::Serializer::new(&mut sink);
            let mut magical_serializer =
                crate::MagicalSerializer::new(typed_serializer);
            magical_serializer.serialize(&3).unwrap();
        }
        assert_eq!(core::str::from_utf8(&sink).unwrap(), "3")
    }
    #[cfg(feature = "yaml")]
    #[test]
    fn serializes_int_to_yaml() {
        let mut sink = Vec::new();
        {
            let typed_serializer =
                serde_yaml::Serializer::new(&mut sink);
            let mut magical_serializer =
                crate::MagicalSerializer::new(typed_serializer);
            magical_serializer.serialize(&2).unwrap();
        }
        assert_eq!(core::str::from_utf8(&sink).unwrap(), "2\n")
    }
    #[cfg(feature = "json")]
    #[test]
    fn serializes_static_obj_to_json() {
        let mut sink = Vec::new();
        {
            let typed_serializer =
                serde_json::Serializer::new(&mut sink);
            let mut magical_serializer =
                crate::MagicalSerializer::new(typed_serializer);
            magical_serializer
                .serialize(&*SERIALIZABLE_STATIC_OBJ)
                .unwrap();
        }
        let sink_content = core::str::from_utf8(&sink).unwrap();
        let correct_answers =
            [r#"{"zero":0,"k":42}"#, r#"{"k":42,"zero":0}"#];
        assert!(correct_answers.contains(&sink_content));
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn serializes_static_obj_to_yaml() {
        let mut sink = Vec::new();
        {
            let typed_serializer =
                serde_yaml::Serializer::new(&mut sink);
            let mut magical_serializer =
                crate::MagicalSerializer::new(typed_serializer);
            magical_serializer
                .serialize(&*SERIALIZABLE_STATIC_OBJ)
                .unwrap();
        }
        let sink_content = core::str::from_utf8(&sink).unwrap();
        let correct_answers =
            ["zero: 0\nk: 42\n", "k: 42\nzero: 0\n"];
        assert!(correct_answers.contains(&sink_content));
    }
}

use std::{borrow::BorrowMut, cell::OnceCell, error::Error, io::Write, ptr::NonNull};

type SeizedWriterHandle<'w> = crate::util::RcRfDynWriter<'w>;

pub struct MagicalSerializer<'w> {
    prefix_for_writes: &'static [u8],
    sufix_for_writes: &'static [u8],
    writer: OnceCell<SeizedWriterHandle<'w>>,
    /// Don't R/W from this address untill dropping the "dependant",
    /// which uniquely borrows the addressed value.
    boxed_dependency: *mut (dyn crate::util::Something + 'w),
    /// Should be dropped first. Should not be taken out of the field.
    erased_dependant: NonNull<dyn erased_serde::Serializer + 'w>,
}
impl<'w> MagicalSerializer<'w> {
    // CRUD-C: Constructors

    pub fn from_direct_impl<T>(typed_serializer: T) -> Self
    where
        T: serde::Serializer + 'w,
    {
        Self {
            prefix_for_writes: b"",
            sufix_for_writes: b"",
            writer: OnceCell::new(),
            boxed_dependency: Box::leak(Box::new(())),
            erased_dependant: unsafe {
                NonNull::new_unchecked(Box::leak(Box::new(<dyn erased_serde::Serializer>::erase(
                    typed_serializer,
                ))))
            },
        }
    }
    pub fn new<T>(typed_serializer: T) -> Self
    where
        T: 'w,
        &'w mut T: serde::Serializer,
    {
        let boxed_dependency: *mut T = Box::leak(Box::new(typed_serializer));
        Self {
            prefix_for_writes: b"",
            sufix_for_writes: b"",
            writer: OnceCell::new(),
            boxed_dependency,
            erased_dependant: unsafe {
                NonNull::new_unchecked(Box::leak(Box::new(<dyn erased_serde::Serializer>::erase(
                    &mut *boxed_dependency,
                ))))
            },
        }
    }
    // CRUD-R: Read settings
    pub fn prefix_for_writes(&self) -> &'static [u8] {
        self.prefix_for_writes
    }
    pub fn sufix_for_writes(&self) -> &'static [u8] {
        self.sufix_for_writes
    }
    // CRUD-U: Update settings
    pub fn set_prefix_for_writes(&mut self, bytes: &'static [u8]) {
        self.prefix_for_writes = bytes;
    }
    pub fn set_sufix_for_writes(&mut self, bytes: &'static [u8]) {
        self.sufix_for_writes = bytes;
    }

    // CRUD-U: Write instructions
    pub fn serialize<'o, O: serde::Serialize + ?Sized>(
        &mut self,
        serializable: &'o O,
    ) -> color_eyre::Result<()> {
        self.write_prefix()?;
        self.serialize_austerely(serializable)?;
        self.write_sufix()?;
        Ok(())
    }
    fn write_prefix(&mut self) -> std::io::Result<usize> {
        let prefix = self.prefix_for_writes();
        if let Some(writer) = self.writer.get_mut() {
            writer.borrow_mut().write(prefix)
        } else if prefix.is_empty() {
            Ok(0)
        } else {
            panic!()
        }
    }
    fn write_sufix(&mut self) -> std::io::Result<usize> {
        let sufix = self.sufix_for_writes();
        if let Some(writer) = self.writer.get_mut() {
            writer.borrow_mut().write(sufix)
        } else if sufix.is_empty() {
            Ok(0)
        } else {
            panic!()
        }
    }
    fn serialize_austerely<'o, O: serde::Serialize + ?Sized>(
        &mut self,
        serializable: &'o O,
    ) -> Result<(), impl Error + 'static> {
        erased_serde::Serialize::erased_serialize(serializable, unsafe {
            // We trust the called function to not take the value out of the field.
            // We know this is the only accessor of all chained serializer's dependencies.
            self.erased_dependant.as_mut()
        })
    }
}
impl<'w> MagicalSerializer<'w> {
    /// Safety: Must seize the writer that is really the one,
    /// that is being used under the hood of serializer(s).
    pub unsafe fn with_seized_writer<'seized_w: 'w>(
        self,
        seized_writer: SeizedWriterHandle<'seized_w>,
    ) -> MagicalSerializer<'seized_w> {
        let prolonged_self: MagicalSerializer<'seized_w> = unsafe { std::mem::transmute(self) };

        prolonged_self
            .writer
            .set(seized_writer)
            .unwrap_or_else(|_| {
                panic!("There must be only one real writer under the hood of serializer(s).")
            });

        prolonged_self
    }
}
impl<'w> Drop for MagicalSerializer<'w> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.erased_dependant.as_ptr()));
            drop(Box::from_raw(self.boxed_dependency));
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
            let typed_serializer = serde_json::Serializer::new(&mut sink);
            let mut magical_serializer = crate::MagicalSerializer::new(typed_serializer);
            magical_serializer.serialize(&3).unwrap();
        }
        assert_eq!(core::str::from_utf8(&sink).unwrap(), "3")
    }
    #[cfg(feature = "yaml")]
    #[test]
    fn serializes_int_to_yaml() {
        let mut sink = Vec::new();
        {
            let typed_serializer = serde_yaml::Serializer::new(&mut sink);
            let mut magical_serializer = crate::MagicalSerializer::new(typed_serializer);
            magical_serializer.serialize(&2).unwrap();
        }
        assert_eq!(core::str::from_utf8(&sink).unwrap(), "2\n")
    }
    #[cfg(feature = "json")]
    #[test]
    fn serializes_static_obj_to_json() {
        let mut sink = Vec::new();
        {
            let typed_serializer = serde_json::Serializer::new(&mut sink);
            let mut magical_serializer = crate::MagicalSerializer::new(typed_serializer);
            magical_serializer
                .serialize(&*SERIALIZABLE_STATIC_OBJ)
                .unwrap();
        }
        let sink_content = core::str::from_utf8(&sink).unwrap();
        let correct_answers = [r#"{"zero":0,"k":42}"#, r#"{"k":42,"zero":0}"#];
        assert!(correct_answers.contains(&sink_content));
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn serializes_static_obj_to_yaml() {
        let mut sink = Vec::new();
        {
            let typed_serializer = serde_yaml::Serializer::new(&mut sink);
            let mut magical_serializer = crate::MagicalSerializer::new(typed_serializer);
            magical_serializer
                .serialize(&*SERIALIZABLE_STATIC_OBJ)
                .unwrap();
        }
        let sink_content = core::str::from_utf8(&sink).unwrap();
        let correct_answers = ["zero: 0\nk: 42\n", "k: 42\nzero: 0\n"];
        assert!(correct_answers.contains(&sink_content));
    }
}

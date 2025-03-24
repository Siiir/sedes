use std::ffi::OsStr;

use crate::DeserializationFormat;

use ext_maps::{FAVOURED_FILE_EXTS, FROM_FILE_EXT};
use strum::EnumProperty as _;
mod ext_maps;

// Built first

pub trait SedeFormat {
    fn from_file_ext_os(file_extension: &OsStr) -> Option<Self>
    where
        Self: Sized;

    fn from_file_ext(file_extension: &str) -> Option<Self>
    where
        Self: Sized;

    fn file_exts(&self) -> impl IntoIterator<Item = &'static str>;

    fn favoured_file_ext(&self) -> &'static str;

    fn alt_file_exts(&self)
    -> impl IntoIterator<Item = &'static str>;
}

impl<D> SedeFormat for D
where
    D: Clone
        + Copy
        + From<DeserializationFormat>
        + for<'a> From<&'a DeserializationFormat>,
    DeserializationFormat: From<D> + for<'a> From<&'a D>,
{
    fn from_file_ext_os(file_extension: &OsStr) -> Option<Self> {
        let file_extension: &str = file_extension.to_str()?;
        Self::from_file_ext(file_extension)
    }
    fn from_file_ext(file_extension: &str) -> Option<Self> {
        let des_fmt = FROM_FILE_EXT.get(file_extension).copied()?;
        Some(des_fmt.into())
    }

    fn file_exts(&self) -> impl IntoIterator<Item = &'static str> {
        let favoured_ext = self.favoured_file_ext();
        let alt_exts = self.alt_file_exts();
        let all_exts = std::iter::once(favoured_ext).chain(alt_exts);
        all_exts
    }

    fn favoured_file_ext(&self) -> &'static str {
        let deserialization_format = self.into();
        FAVOURED_FILE_EXTS
            .get_by_left(&deserialization_format)
            .expect("every format should have a file extension")
    }

    fn alt_file_exts(
        &self,
    ) -> impl IntoIterator<Item = &'static str> {
        DeserializationFormat::from(self)
            .get_str("alt_file_exts")
            .into_iter()
            .flat_map(|exts| exts.split(","))
            .map(str::trim)
    }
}

#[cfg(test)]
mod test {
    mod yaml {
        use std::collections::HashSet;

        use crate::{DeserializationFormat, SedeFormat};

        #[test]
        fn lists_yml_and_yaml_file_exts() {
            let yaml_file_exts: HashSet<&str> =
                DeserializationFormat::Yaml
                    .file_exts()
                    .into_iter()
                    .collect();

            let wanted_file_exts: HashSet<&str> =
                ["yml", "yaml"].into();
            assert!(yaml_file_exts.is_superset(&wanted_file_exts));
        }

        #[test]
        fn constructs_from_yml_file_ext() {
            let sede_fmt =
                DeserializationFormat::from_file_ext("yml");
            assert_eq!(sede_fmt, Some(DeserializationFormat::Yaml));
        }

        #[test]
        fn constructs_from_yaml_file_ext() {
            let sede_fmt =
                DeserializationFormat::from_file_ext("yaml");
            assert_eq!(sede_fmt, Some(DeserializationFormat::Yaml));
        }
    }
}

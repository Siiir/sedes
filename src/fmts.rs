use std::{
    io::{Read, Write},
    sync::LazyLock,
};

use bimap::BiHashMap;
use strum::{EnumProperty, VariantArray};

static FILE_EXTENSIONS: LazyLock<
    BiHashMap<SerializationFormat, &str>,
> = LazyLock::new(|| {
    SerializationFormat::VARIANTS
        .iter()
        .map(|&variant| {
            (
                variant,
                variant.get_str("file_ext").expect(
                    "every variant should have \"file_ext\" property",
                ),
            )
        })
        .collect()
});

#[derive(
    Clone,
    Copy,
    strum::EnumString,
    strum::VariantArray,
    strum::EnumProperty,
    strum::IntoStaticStr,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SerializationFormat {
    #[cfg(feature = "json")]
    #[strum(serialize = "JSON", props(file_ext = "json"))]
    Json,

    #[cfg(feature = "yaml")]
    #[strum(serialize = "YAML", props(file_ext = "yml"))]
    Yaml,

    #[cfg(feature = "cbor")]
    #[strum(serialize = "CBOR", props(file_ext = "cbor"))]
    Cbor,

    #[cfg(feature = "rmp")]
    #[strum(serialize = "RMP", props(file_ext = "rmp"))]
    Rmp,

    #[cfg(feature = "bincode")]
    #[strum(serialize = "Bincode", props(file_ext = "bincode"))]
    Bincode,

    #[cfg(feature = "pickle")]
    #[strum(serialize = "Pickle", props(file_ext = "pkl"))]
    Pickle,
}

impl SerializationFormat {
    pub fn serializer<'w, W: Write + 'w>(
        self,
        writer: W,
    ) -> crate::MagicalSerializer<'w> {
        #[allow(unused_macros)]
        macro_rules! wrap {
            ($serializer:ty $(, $arg:expr)*) => {
                crate::MagicalSerializer::new(<$serializer>::new(writer, $($arg,)*))
            };
        }

        match self {
            #[cfg(feature = "json")]
            Self::Json => wrap!(serde_json::Serializer::<W>),

            #[cfg(feature = "yaml")]
            Self::Yaml => wrap!(serde_yaml::Serializer::<W>),

            #[cfg(feature = "cbor")]
            Self::Cbor => crate::MagicalSerializer::new(
                serde_cbor::Serializer::new(
                    serde_cbor::ser::IoWrite::new(writer),
                ),
            ),

            #[cfg(feature = "rmp")]
            Self::Rmp => wrap!(rmp_serde::Serializer::<W>),

            #[cfg(feature = "bincode")]
            Self::Bincode => wrap!(
                bincode::Serializer::<W, _>,
                bincode::DefaultOptions::new()
            ),

            #[cfg(feature = "pickle")]
            Self::Pickle => wrap!(
                serde_pickle::Serializer::<W>,
                serde_pickle::SerOptions::default()
            ),
        }
    }
    pub fn deserializer<'r, R: Read + 'r>(
        self,
        reader: R,
    ) -> crate::MagicalDeserializer<'r> {
        match self {
            #[cfg(feature = "json")]
            Self::Json => {
                crate::MagicalDeserializer::new(
                    serde_json::Deserializer::from_reader(reader),
                )
            }

            #[cfg(feature = "yaml")]
            Self::Yaml => {
                crate::MagicalDeserializer::from_direct_impl(
                    serde_yaml::Deserializer::from_reader(reader),
                )
            }

            #[cfg(feature = "cbor")]
            Self::Cbor => crate::MagicalDeserializer::new(
                serde_cbor::Deserializer::new(
                    serde_cbor::de::IoRead::new(reader),
                ),
            ),

            #[cfg(feature = "rmp")]
            Self::Rmp => crate::MagicalDeserializer::new(
                rmp_serde::Deserializer::new(reader),
            ),

            #[cfg(feature = "bincode")]
            Self::Bincode => crate::MagicalDeserializer::new(
                bincode::Deserializer::with_reader(
                    reader,
                    bincode::DefaultOptions::new(),
                ),
            ),

            #[cfg(feature = "pickle")]
            Self::Pickle => crate::MagicalDeserializer::new(
                serde_pickle::Deserializer::new(
                    reader,
                    serde_pickle::DeOptions::default(),
                ),
            ),
        }
    }
}
impl SerializationFormat {
    pub fn from_file_ext(file_extension: &str) -> Option<Self> {
        FILE_EXTENSIONS.get_by_right(file_extension).copied()
    }

    pub fn file_ext(self) -> &'static str {
        FILE_EXTENSIONS
            .get_by_left(&self)
            .expect("every format should have a file extension")
    }
}

#[cfg(test)]
mod test {
    mod file_extensions {
        use std::sync::LazyLock;

        #[test]
        fn lazy_loads_correctly() {
            LazyLock::force(&super::super::FILE_EXTENSIONS);
        }
    }
}

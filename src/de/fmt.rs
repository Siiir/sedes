use std::{
    io::{Read, Write},
    sync::LazyLock,
};

use bimap::BiHashMap;
use strum::{EnumProperty, VariantArray};

use crate::SerializationFormat;

static FILE_EXTENSIONS: LazyLock<
    BiHashMap<DeserializationFormat, &str>,
> = LazyLock::new(|| {
    DeserializationFormat::VARIANTS
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
    // CRUD-C:
    Clone,
    Copy,
    strum::EnumString,
    // CRUD-R: Type properties
    strum::VariantArray,
    strum::VariantNames,
    // CRUD-R: Instance properties
    strum::EnumProperty,
    strum::EnumIs,
    strum::IntoStaticStr,
    // CRUD-R: Displayers
    strum::Display,
    Debug,
    // CRUD-R: Equivalence
    PartialEq,
    Eq,
    Hash,
)]
#[cfg(feature = "json")]
#[derive(Default)]
pub enum DeserializationFormat {
    #[cfg(feature = "json")]
    #[strum(serialize = "JSON", props(file_ext = "json"))]
    #[default]
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

impl DeserializationFormat {
    pub fn serializer<'w, W: Write + 'w>(
        self,
        writer: W,
    ) -> crate::MagicalSerializer<'w> {
        SerializationFormat::from(self).serializer(writer)
    }

    pub fn deserializer<'r, R: Read + 'r>(
        self,
        reader: R,
    ) -> crate::MagicalDeserializer<'r> {
        match self {
            #[cfg(feature = "json")]
            Self::Json => crate::MagicalDeserializer::new(
                serde_json::Deserializer::from_reader(reader),
            ),

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
impl DeserializationFormat {
    pub fn from_file_ext(file_extension: &str) -> Option<Self> {
        FILE_EXTENSIONS.get_by_right(file_extension).copied()
    }

    pub fn file_ext(self) -> &'static str {
        FILE_EXTENSIONS
            .get_by_left(&self)
            .expect("every format should have a file extension")
    }
}

// CRUD-C:

impl From<SerializationFormat> for DeserializationFormat {
    fn from(value: SerializationFormat) -> Self {
        (&value).into()
    }
}

impl From<&SerializationFormat> for DeserializationFormat {
    fn from(value: &SerializationFormat) -> Self {
        use SerializationFormat as SF;
        match value {
            SF::PrettyJson => Self::Json,
            SF::Json => Self::Json,
            SF::Yaml => Self::Yaml,
            SF::Cbor => Self::Cbor,
            SF::Rmp => Self::Rmp,
            SF::Bincode => Self::Bincode,
            SF::Pickle => Self::Pickle,
        }
    }
}

impl From<&Self> for DeserializationFormat {
    fn from(value: &Self) -> Self {
        *value
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
    #[cfg(feature = "pickle")]
    mod pickle {
        use rand::Rng;

        type Serializable = (i64, f32, bool);

        #[test]
        fn magic_sede_should_work_like_static()
        -> color_eyre::Result<()> {
            let mut rng = rand::rng();
            let serializable: Serializable =
                Rng::random(&mut rng);

            let static_pickle_bytes = serde_pickle::to_vec(
                &serializable,
                serde_pickle::SerOptions::new(),
            )
            .unwrap();
            assert_eq!(
            crate::deserialize_magically::<_, _, Serializable>(
                static_pickle_bytes.as_slice(),
                "Pickle"
            )
            .unwrap(),
            serializable
        );

            let mut dynamic_pickle_bytes = Vec::<u8>::new();
            crate::serialize_magically(
                &mut dynamic_pickle_bytes,
                "Pickle",
                &serializable,
            )
            .unwrap();

            assert_eq!(dynamic_pickle_bytes, static_pickle_bytes);

            Ok(())
        }
    }
}

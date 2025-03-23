use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::SerializationFormat;

#[derive(
    // CRUD-C:
    Deserialize,
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
    // CRUD-R: Misc
    Serialize,
)]
#[cfg(feature = "json")]
#[derive(Default)]
pub enum DeserializationFormat {
    #[cfg(feature = "json")]
    #[strum(serialize = "JSON", props(file_ext = "json"))]
    #[default]
    Json,

    #[cfg(feature = "yaml")]
    #[strum(
        serialize = "YAML",
        props(file_ext = "yml", alt_file_exts = "yaml")
    )]
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
mod test;

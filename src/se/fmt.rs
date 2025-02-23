use std::io::{Read, Write};

use crate::DeserializationFormat;

#[derive(
    // CRUD-C: Constructors
    Clone,
    Copy,
    strum::EnumString,
    // CRUD-R: Properties
    strum::VariantArray,
    strum::VariantNames,
    strum::EnumIs,
    // CRUD-R: Displayers
    strum::IntoStaticStr,
    strum::Display,
    Debug,
    // CRUD-R: Equivalence
    PartialEq,
    Eq,
    Hash,
)]
#[cfg(feature = "json")]
#[derive(Default)]
pub enum SerializationFormat {
    #[cfg(feature = "json")]
    #[strum(serialize = "JSON-pretty")]
    #[default]
    PrettyJson,

    #[cfg(feature = "json")]
    #[strum(serialize = "JSON")]
    Json,

    #[cfg(feature = "yaml")]
    #[strum(serialize = "YAML")]
    Yaml,

    #[cfg(feature = "cbor")]
    #[strum(serialize = "CBOR")]
    Cbor,

    #[cfg(feature = "rmp")]
    #[strum(serialize = "RMP")]
    Rmp,

    #[cfg(feature = "bincode")]
    #[strum(serialize = "Bincode")]
    Bincode,

    #[cfg(feature = "pickle")]
    #[strum(serialize = "Pickle")]
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
            Self::PrettyJson => crate::MagicalSerializer::new(
                serde_json::Serializer::pretty(writer),
            ),
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
            Self::Pickle => {
                let protocol_header = &[128, 3];
                let stop_opcode = b".";
                let writer = crate::util::RcRfWriter::from(writer);

                let m = crate::MagicalSerializer::new(
                    serde_pickle::Serializer::new(
                        writer.clone(),
                        serde_pickle::SerOptions::default(),
                    ),
                );
                let mut m = unsafe {
                    m.with_seized_writer(writer.with_dyn_write())
                };

                m.set_prefix_for_writes(protocol_header);
                m.set_sufix_for_writes(stop_opcode);
                m
            }
        }
    }

    pub fn deserializer<'r, R: Read + 'r>(
        self,
        reader: R,
    ) -> crate::MagicalDeserializer<'r> {
        DeserializationFormat::from(self).deserializer(reader)
    }
}
impl SerializationFormat {
    pub fn from_file_ext(file_extension: &str) -> Option<Self> {
        DeserializationFormat::from_file_ext(file_extension).map(Into::into)        
    }

    pub fn file_ext(self) -> &'static str {
        DeserializationFormat::file_ext(self.into())
    }
}


// CRUD-C:

impl From<DeserializationFormat> for SerializationFormat {
    fn from(value: DeserializationFormat) -> Self {
        (&value).into()
    }
}

impl From<&DeserializationFormat> for SerializationFormat {
    fn from(value: &DeserializationFormat) -> Self {
        use DeserializationFormat as DF;
        match value {
            DF::Json => Self::PrettyJson,
            DF::Yaml => Self::Yaml,
            DF::Cbor => Self::Cbor,
            DF::Rmp => Self::Rmp,
            DF::Bincode => Self::Bincode,
            DF::Pickle => Self::Pickle,
        }
    }
}

impl From<&SerializationFormat> for SerializationFormat {
    fn from(value: &SerializationFormat) -> Self {
        *value
    }
}

// CRUD-R:

impl strum::EnumProperty for SerializationFormat {
    fn get_str(&self, prop: &str) -> Option<&'static str> {
        DeserializationFormat::from(self).get_str(prop)
    }

    fn get_int(&self, prop: &str) -> Option<i64> {
        DeserializationFormat::from(self).get_int(prop)
    }

    fn get_bool(&self, prop: &str) -> Option<bool> {
        DeserializationFormat::from(self).get_bool(prop)
    }
}


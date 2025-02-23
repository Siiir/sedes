use std::io::Read;

pub mod magical;

pub fn make_deserializer<
    'r,
    R: Read + 'r,
    F: TryInto<crate::DeserializationFormat>,
>(
    reader: R,
    format: F,
) -> Result<crate::MagicalDeserializer<'r>, F::Error> {
    format.try_into().map(|fmt| fmt.deserializer(reader))
}

pub fn deserialize_magically<'r, R, F, O>(
    reader: R,
    format: F,
) -> color_eyre::Result<O>
where
    R: Read + 'r,
    F: TryInto<crate::DeserializationFormat>,
    color_eyre::Report: From<F::Error>,
    O: serde::de::DeserializeOwned,
{
    let mut deserializer: crate::MagicalDeserializer<'r> =
        make_deserializer(reader, format)?;
    Ok(deserializer.deserialize()?)
}

pub mod fmt;

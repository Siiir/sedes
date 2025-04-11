use std::io::Write;

pub mod magical;

pub mod fmt;

pub mod fs;

pub fn make_serializer<'w, W: Write + 'w, F: TryInto<crate::SerializationFormat>>(
    writer: W,
    format: F,
) -> Result<crate::MagicalSerializer<'w>, F::Error> {
    format.try_into().map(|fmt| fmt.serializer(writer))
}

/// Dynamically serialize any `serde::Serialize` object.
/// # Examples
///
/// ```rust
/// #[cfg(features="yaml")]
/// {
///     let mut writer = Vec::<u8>::new();
///     sedes::serialize_magically(&mut writer, "YAML", &42).unwrap();
///     let output = core::str::from_utf8(writer.as_slice()).unwrap();
///     assert_eq!(output, "42\n");
/// }
/// ```
///    
/// ```rust
/// #[cfg(features="json")]
/// {
///     let mut writer = Vec::<u8>::new();
///     sedes::serialize_magically(&mut writer, "JSON", &None::<f64>).unwrap();
///     let output = core::str::from_utf8(writer.as_slice()).unwrap();
///     assert_eq!(output, "null");
/// }
/// ```
pub fn serialize_magically<'w, 'o, W, F, O>(
    writer: W,
    format: F,
    serializable: &O,
) -> color_eyre::Result<()>
where
    W: Write + 'w,
    F: TryInto<crate::SerializationFormat>,
    color_eyre::Report: From<F::Error>,
    O: serde::Serialize + ?Sized + 'o,
{
    let mut serializer: crate::MagicalSerializer<'w> = make_serializer(writer, format)?;
    serializer.serialize(serializable)?;
    Ok(())
}

#[cfg(test)]
mod test;

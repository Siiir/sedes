use std::io::Write;

pub mod magical;

pub fn make_serializer<
    'w,
    W: Write + 'w,
    F: TryInto<crate::SerializationFormat>,
>(
    writer: W,
    format: F,
) -> Result<crate::MagicalSerializer<'w>, F::Error> {
    format.try_into().map(|fmt| fmt.serializer(writer))
}

/// Dynamically serialize any `serde::Serialize` object.
/// # Example
/// ```rust
/// let mut writer = Vec::<u8>::new();
/// serialize_magically(&mut writer, "YAML", &42).unwrap();
/// let output = core::str::from_utf8(writer.as_slice()).unwrap();
/// assert_eq!(output, "42");
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
    let mut dynamic_serializer: crate::MagicalSerializer<'w> =
        make_serializer(writer, format)?;
    dynamic_serializer.serialize(serializable)?;
    Ok(())
}

#[cfg(test)]
mod test {
    mod dyn_serialize_static {
        use strum::VariantArray;

        use super::super::serialize_magically;
        #[test]
        fn succeeds_with_existing_fmt() -> color_eyre::Result<()> {
            serialize_magically(
                std::io::sink(),
                crate::SerializationFormat::VARIANTS[0],
                &(),
            )
        }
        #[test]
        fn fails_with_non_existing_fmt() {
            assert!(serialize_magically(std::io::sink(), "Json", &())
                .is_err())
        }
        #[cfg(feature = "json")]
        #[test]
        fn serializes_json() {
            let mut writer = Vec::<u8>::new();
            serialize_magically(&mut writer, "JSON", &42).unwrap();
            assert_eq!(
                core::str::from_utf8(writer.as_slice()).unwrap(),
                "42"
            )
        }
    }
}

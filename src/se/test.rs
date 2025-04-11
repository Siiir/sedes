mod dyn_serialize_static {
    use strum::VariantArray;

    use super::super::serialize_magically;
    #[test]
    fn succeeds_with_existing_fmt() -> color_eyre::Result<()> {
        serialize_magically(
            std::io::sink(),
            crate::DeserializationFormat::VARIANTS[0],
            &(),
        )
    }
    #[test]
    fn fails_with_non_existing_fmt() {
        assert!(serialize_magically(std::io::sink(), "Json", &()).is_err())
    }
    #[cfg(feature = "json")]
    #[test]
    fn serializes_json() {
        let mut writer = Vec::<u8>::new();
        serialize_magically(&mut writer, "JSON", &42).unwrap();
        assert_eq!(core::str::from_utf8(writer.as_slice()).unwrap(), "42")
    }
}

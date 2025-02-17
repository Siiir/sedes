pub use {
    de::{
        deserialize_magically, magical::MagicalDeserializer,
        make_deserializer,
    },
    fmts::SerializationFormat,
    se::{
        magical::MagicalSerializer, make_serializer,
        serialize_magically,
    },
};
pub mod de;
pub mod fmts;
pub mod se;

// Private modules.
mod util;

#[cfg(test)]
pub mod test {
    use color_eyre::eyre::Context;
    use rand::Rng;
    use strum::VariantArray;

    type Serializable = (i64, f32, bool);

    #[test]
    fn sede_bijectivity() -> color_eyre::Result<()> {
        for fmt in crate::SerializationFormat::VARIANTS
        {
            for _ in 0..5 {
                test_bijectivity_for(fmt)
                    .with_context(|| format!("failed for {fmt}"))?;
            }
        }
        Ok(())
    }

    fn test_bijectivity_for(
        fmt: &crate::SerializationFormat,
    ) -> color_eyre::Result<()> {
        let mut rng = rand::rng();
        let mut sink = Vec::<u8>::new();

        let serializable: Serializable = Rng::random(&mut rng);

        sink.clear();
        crate::serialize_magically(&mut sink, fmt, &serializable)?;
        let deserialized: Serializable =
            crate::deserialize_magically(sink.as_slice(), fmt)?;
        assert_eq!(deserialized, serializable);

        Ok(())
    }
}

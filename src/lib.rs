use std::{io::Read, io::Write};
pub use {
    de::{
        deserialize_magically, fmt::DeserializationFormat,
        magical::MagicalDeserializer, make_deserializer,
    },
    se::{
        fmt::SerializationFormat, magical::MagicalSerializer,
        make_serializer, serialize_magically,
    },
};

pub mod de;
pub mod se;
pub use sede::fmt::SedeFormat;
pub mod sede;
mod util;

pub fn translate_magically<'r, 'w, T, R, W, I, O>(
    reader: R,
    input_fmt: I,
    writer: W,
    output_fmt: O,
) -> color_eyre::Result<()>
where
    T: serde::de::DeserializeOwned + serde::Serialize + ?Sized,
    R: Read + 'r,
    W: Write + 'w,
    I: TryInto<crate::DeserializationFormat>,
    color_eyre::Report: From<I::Error>,
    O: TryInto<crate::SerializationFormat>,
    color_eyre::Report: From<O::Error>,
{
    let value: T = deserialize_magically(reader, input_fmt)?;
    serialize_magically(writer, output_fmt, &value)?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use color_eyre::eyre::Context;
    use rand::Rng;
    use strum::VariantArray;

    type Serializable = (i64, f32, bool);

    #[test]
    fn sede_bijectivity() -> color_eyre::Result<()> {
        for fmt in crate::DeserializationFormat::VARIANTS {
            for _ in 0..5 {
                test_bijectivity_for(fmt)
                    .with_context(|| format!("failed for {fmt}"))?;
            }
        }
        Ok(())
    }

    fn test_bijectivity_for(
        fmt: &crate::DeserializationFormat,
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

#[cfg(feature = "pickle")]
mod pickle {
    use rand::Rng;

    type Serializable = (i64, f32, bool);

    #[test]
    fn magic_de_should_revert_static_se() -> color_eyre::Result<()> {
        let mut rng = rand::rng();
        let serializable: Serializable = Rng::random(&mut rng);

        // Static serialization
        let static_pickle_bytes =
            serde_pickle::to_vec(&serializable, serde_pickle::SerOptions::new()).unwrap();
        // Magic deserialization

        let deserialized = crate::deserialize_magically::<_, _, Serializable>(
            static_pickle_bytes.as_slice(),
            "Pickle",
        )
        .unwrap();
        assert_eq!(deserialized, serializable);

        Ok(())
    }
}

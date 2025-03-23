#[cfg(feature = "pickle")]
mod pickle {
    use rand::Rng;

    type Serializable = (i64, f32, bool);

    #[test]
    fn magic_should_work_like_static() -> color_eyre::Result<()> {
        let mut rng = rand::rng();
        let serializable: Serializable = Rng::random(&mut rng);

        // Static
        let static_pickle_bytes = serde_pickle::to_vec(
            &serializable,
            serde_pickle::SerOptions::new(),
        )
        .unwrap();

        // Magic
        let mut dynamic_pickle_bytes = Vec::<u8>::new();
        crate::serialize_magically(
            &mut dynamic_pickle_bytes,
            "Pickle",
            &serializable,
        )
        .unwrap();

        // Comparison
        assert_eq!(dynamic_pickle_bytes, static_pickle_bytes);

        Ok(())
    }
}

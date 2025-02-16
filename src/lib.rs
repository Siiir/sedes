mod util {
    pub trait Something {}
    impl<T> Something for T {}
}
pub use sede::{
    fmts::SerializationFormat,
    se::{
        serialize_magically, magical::MagicalSerializer,
        make_serializer,
    },
};
pub mod sede;

pub use {
    fmts::SerializationFormat,
    se::{
        magical::MagicalSerializer, make_serializer,
        serialize_magically,
    },
    de::{magical::MagicalDeserializer},
};
pub mod se;
pub mod de;
pub mod fmts;

// Private modules.
mod util {
    pub trait Something {}
    impl<T> Something for T {}
}

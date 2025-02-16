pub use {
    fmts::SerializationFormat,
    se::{
        magical::MagicalSerializer, make_serializer, serialize_magically
    },
};
pub mod se;
pub mod fmts;

// Private modules.
mod util {
    pub trait Something {}
    impl<T> Something for T {}
}

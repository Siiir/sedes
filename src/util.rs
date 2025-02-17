pub trait Something {}
impl<T> Something for T {}

pub use rc_rf::{RcRfDynWriter, RcRfWriter};
pub mod rc_rf {
    use std::{cell::RefCell, io::Write, rc::Rc};

    type RcRf<W> = Rc<RefCell<W>>;
    type RcRfDynWrite<'w> = RcRf<dyn Write + 'w>;

    pub type RcRfDynWriter<'w> = RcRfWriter<dyn Write + 'w>;

    #[derive(Debug)]
    pub struct RcRfWriter<W: ?Sized>(RcRf<W>);

    impl<'w, W: Write + 'w> RcRfWriter<W> {
        pub fn with_dyn_write(self) -> RcRfDynWriter<'w> {
            let inner: RcRfDynWrite = self.0;
            inner.into()
        }
    }

    impl<W> From<W> for RcRfWriter<W> {
        fn from(value: W) -> Self {
            RefCell::new(value).into()
        }
    }
    impl<W> From<RefCell<W>> for RcRfWriter<W> {
        fn from(value: RefCell<W>) -> Self {
            Rc::new(value).into()
        }
    }
    impl<W: ?Sized> From<RcRf<W>> for RcRfWriter<W> {
        fn from(value: RcRf<W>) -> Self {
            Self(value)
        }
    }
    impl<'w, W: Write + 'w> From<RcRfWriter<W>>
        for RcRfWriter<dyn Write + 'w>
    {
        fn from(value: RcRfWriter<W>) -> Self {
            value.with_dyn_write()
        }
    }

    impl<W: ?Sized> Clone for RcRfWriter<W> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<W: ?Sized + Write> Write for RcRfWriter<W> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.borrow_mut().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.borrow_mut().flush()
        }
    }
}

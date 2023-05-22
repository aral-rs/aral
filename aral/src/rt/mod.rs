use crate::imp;
use std::future::Future;

pub struct Builder(imp::rt::Builder);

impl Builder {
    pub fn new() -> Self {
        Self(imp::rt::Builder::new())
    }

    pub fn build(self) -> std::io::Result<Runtime> {
        Ok(Runtime(self.0.build()?))
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Runtime(imp::rt::Runtime);

impl Runtime {
    pub fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.block_on(future)
    }
}

use crate::imp;
use std::future::Future;

pub struct Builder(imp::rt::Builder);

impl Builder {
    pub fn new() -> Self {
        Self(imp::rt::Builder::new())
    }

    pub fn build(self) -> std::io::Result<Runtime> {
        self.0.build().map(Runtime)
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

pub struct JoinHandle<T>(imp::rt::JoinHandle<T>);

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(imp::rt::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(imp::rt::spawn_blocking(f))
}

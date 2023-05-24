use std::{future::Future, io::Result};

pub(crate) struct Builder;

impl Builder {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn build(self) -> Result<Runtime> {
        Ok(Runtime(async_std::task::Builder::new()))
    }
}

pub(crate) struct Runtime(async_std::task::Builder);

impl Runtime {
    pub(crate) fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.blocking(future)
    }
}

pub struct JoinHandle<T>(async_std::task::JoinHandle<T>);

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(async_std::task::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(async_std::task::spawn_blocking(f))
}

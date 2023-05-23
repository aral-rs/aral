use std::future::Future;

#[allow(dead_code)]
pub(crate) struct CurrentThreadBuilder(tokio::runtime::Builder);

#[allow(dead_code)]
impl CurrentThreadBuilder {
    pub(crate) fn new() -> Self {
        let mut builder = tokio::runtime::Builder::new_current_thread();
        builder.enable_all();
        Self(builder)
    }

    pub(crate) fn build(mut self) -> std::io::Result<Runtime> {
        Ok(Runtime(self.0.build()?))
    }
}

#[allow(dead_code)]
pub(crate) struct MultiThreadBuilder(tokio::runtime::Builder);

#[allow(dead_code)]
impl MultiThreadBuilder {
    pub(crate) fn new() -> Self {
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.enable_all();
        Self(builder)
    }

    pub(crate) fn build(mut self) -> std::io::Result<Runtime> {
        Ok(Runtime(self.0.build()?))
    }
}

pub(crate) struct Runtime(tokio::runtime::Runtime);

impl Runtime {
    pub(crate) fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.block_on(future)
    }
}

pub struct JoinHandle<T>(tokio::task::JoinHandle<T>);

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(tokio::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(tokio::task::spawn_blocking(f))
}

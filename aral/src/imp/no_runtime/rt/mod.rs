use std::{future::Future, marker::PhantomData};

pub(crate) struct Builder;

impl Builder {
    pub(crate) fn new() -> Self {
        no_runtime_specified!();
    }

    pub(crate) fn build(self) -> std::io::Result<Runtime> {
        no_runtime_specified!();
    }
}

pub(crate) struct Runtime;

impl Runtime {
    pub(crate) fn block_on<F: Future>(self, _future: F) -> F::Output {
        no_runtime_specified!();
    }
}

pub struct JoinHandle<T>(PhantomData<T>);

#[inline]
pub fn spawn<T: Send + 'static>(
    _future: impl Future<Output = T> + Send + 'static,
) -> JoinHandle<T> {
    no_runtime_specified!();
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(_f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    no_runtime_specified!();
}

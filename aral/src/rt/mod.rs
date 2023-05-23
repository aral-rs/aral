use crate::imp;
use std::{future::Future, marker::PhantomData};

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

pub struct JoinHandle<T>(PhantomData<T>);

pub fn spawn<T>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    todo!()
}

pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    todo!()
}

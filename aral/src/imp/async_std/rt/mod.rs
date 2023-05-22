use std::future::Future;

pub(crate) struct Builder(async_std::task::Builder);

impl Builder {
    pub(crate) fn new() -> Self {
        let builder = async_std::task::Builder::new();
        Self(builder)
    }

    pub(crate) fn build(self) -> std::io::Result<Runtime> {
        Ok(Runtime(self.0))
    }
}

pub(crate) struct Runtime(async_std::task::Builder);

impl Runtime {
    pub(crate) fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.blocking(future)
    }
}

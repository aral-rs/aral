use std::future::Future;

pub(crate) struct Builder(tokio::runtime::Builder);

impl Builder {
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

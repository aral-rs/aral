use std::future::Future;

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

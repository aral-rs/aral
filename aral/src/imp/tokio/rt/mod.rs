use std::future::Future;

pub struct Builder(tokio::runtime::Builder);

impl Builder {
    pub fn new() -> Self {
        let mut builder = {
            #[cfg(feature = "runtime-tokio-multi-thread")]
            {
                tokio::runtime::Builder::new_multi_thread()
            }

            #[cfg(feature = "runtime-tokio-current-thread")]
            {
                tokio::runtime::Builder::new_current_thread()
            }
        };

        builder.enable_all();

        Self(builder)
    }

    pub fn build(mut self) -> std::io::Result<Runtime> {
        self.0.build().map(Runtime)
    }
}

pub struct Runtime(tokio::runtime::Runtime);

impl Runtime {
    pub fn block_on<F: Future>(self, future: F) -> F::Output {
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

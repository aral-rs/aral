use std::{
    any::Any,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    result,
    task::{Context, Poll},
    time::Duration,
};

#[inline]
pub async fn sleep(_duration: Duration) {
    no_runtime_specified!();
}

pub struct JoinHandle<T>(PhantomData<T>);

impl<T> Unpin for JoinHandle<T> {}

impl<T> JoinHandle<T> {
    pub async fn cancel(self) -> Option<T> {
        no_runtime_specified!();
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = result::Result<T, Box<dyn Any + Send + 'static>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        no_runtime_specified!();
    }
}

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

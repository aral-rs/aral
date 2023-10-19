use std::{
    any::Any,
    future::Future,
    pin::{pin, Pin},
    result,
    task::{Context, Poll},
    time::Duration,
};

#[inline]
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await
}

pub struct JoinHandle<T>(tokio::task::JoinHandle<T>);

impl<T> JoinHandle<T> {
    pub async fn cancel(self) -> Option<T> {
        self.0.abort();
        self.0.await.ok()
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = result::Result<T, Box<dyn Any + Send + 'static>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        pin!(&mut self.0)
            .poll(cx)
            .map(|r| r.map_err(|err| err.into_panic()))
    }
}

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(tokio::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(tokio::task::spawn_blocking(f))
}

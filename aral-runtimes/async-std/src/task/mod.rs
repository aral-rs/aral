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
    async_std::task::sleep(duration).await
}

pub struct JoinHandle<T>(async_std::task::JoinHandle<T>);

impl<T> JoinHandle<T> {
    #[inline]
    pub async fn cancel(self) -> Option<T> {
        self.0.cancel().await
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = result::Result<T, Box<dyn Any + Send + 'static>>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        pin!(&mut self.0).poll(cx).map(|t| Ok(t))
    }
}

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(async_std::task::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(async_std::task::spawn_blocking(f))
}

use crate::imp;
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
    imp::task::sleep(duration).await
}

pub struct JoinHandle<T>(imp::task::JoinHandle<T>);

impl<T> JoinHandle<T> {
    #[inline]
    pub async fn cancel(self) -> Option<T> {
        self.0.cancel().await
    }
}

pub type Result<T> = result::Result<T, Box<dyn Any + Send + 'static>>;

impl<T> Future for JoinHandle<T> {
    type Output = Result<T>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(pin!(&mut self.0), cx)
    }
}

#[inline]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    JoinHandle(imp::task::spawn(future))
}

#[inline]
pub fn spawn_blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> JoinHandle<T> {
    JoinHandle(imp::task::spawn_blocking(f))
}

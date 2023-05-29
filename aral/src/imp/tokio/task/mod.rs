// Copyright 2023 ARAL Development Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    any::Any,
    future::Future,
    pin::{pin, Pin},
    result,
    task::{Context, Poll},
    time::Duration,
};

#[inline]
pub(crate) async fn sleep(duration: Duration) {
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

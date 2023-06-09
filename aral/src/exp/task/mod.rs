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

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
    io::Result,
    pin::{pin, Pin},
    result,
    task::{Context, Poll},
};

pub(crate) struct Builder;

impl Builder {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn build(self) -> Result<Runtime> {
        Ok(Runtime(async_std::task::Builder::new()))
    }
}

pub(crate) struct Runtime(async_std::task::Builder);

impl Runtime {
    pub(crate) fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.blocking(future)
    }
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

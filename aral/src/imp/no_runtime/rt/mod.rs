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
    marker::PhantomData,
    pin::Pin,
    result,
    task::{Context, Poll},
};

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

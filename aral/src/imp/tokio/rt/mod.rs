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

use cfg_if::cfg_if;
use std::future::Future;

pub struct Builder(tokio::runtime::Builder);

impl Builder {
    cfg_if! {
        if #[cfg(feature = "runtime-tokio-multi-thread")] {
            #[inline]
            fn new_builder() -> tokio::runtime::Builder {
                tokio::runtime::Builder::new_multi_thread()
            }

        } else if #[cfg(feature = "runtime-tokio-current-thread")] {
            #[inline]
            fn new_builder() -> tokio::runtime::Builder {
                tokio::runtime::Builder::new_current_thread()
            }
        }
    }

    pub fn new() -> Self {
        let mut builder = Self::new_builder();
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

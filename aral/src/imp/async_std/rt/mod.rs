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

use std::{future::Future, io::Result};

pub struct Builder {
    thread_name: Option<String>,
    worker_threads: Option<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            thread_name: None,
            worker_threads: None,
        }
    }

    pub fn thread_name(mut self, name: impl Into<String>) -> Self {
        self.thread_name = Some(name.into());
        self
    }

    pub fn worker_threads(mut self, val: usize) -> Self {
        self.worker_threads = Some(val);
        self
    }

    pub fn build(self) -> Result<Runtime> {
        if let Some(thread_name) = self.thread_name {
            std::env::set_var("ASYNC_STD_THREAD_NAME", thread_name);
        }
        if let Some(worker_threads) = self.worker_threads {
            std::env::set_var("ASYNC_STD_THREAD_COUNT", worker_threads.to_string());
        }
        Ok(Runtime(async_std::task::Builder::new()))
    }
}

pub struct Runtime(async_std::task::Builder);

impl Runtime {
    pub fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.blocking(future)
    }
}

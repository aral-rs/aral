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

use std::future::Future;

pub struct Builder;

impl Builder {
    pub fn new() -> Self {
        no_runtime_specified!();
    }

    pub fn thread_name(self, _name: impl Into<String>) -> Self {
        no_runtime_specified!();
    }

    pub fn worker_threads(self, _val: usize) -> Self {
        no_runtime_specified!();
    }

    pub fn build(self) -> std::io::Result<Runtime> {
        no_runtime_specified!();
    }
}

pub struct Runtime;

impl Runtime {
    pub fn block_on<F: Future>(self, _future: F) -> F::Output {
        no_runtime_specified!();
    }
}

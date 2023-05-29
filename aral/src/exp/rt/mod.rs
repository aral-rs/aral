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
use std::future::Future;

pub struct Builder(imp::rt::Builder);

impl Builder {
    pub fn new() -> Self {
        Self(imp::rt::Builder::new())
    }

    pub fn build(self) -> std::io::Result<Runtime> {
        self.0.build().map(Runtime)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Runtime(imp::rt::Runtime);

impl Runtime {
    pub fn block_on<F: Future>(self, future: F) -> F::Output {
        self.0.block_on(future)
    }
}

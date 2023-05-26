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

use std::{io::Result, net::Shutdown, path::Path};

pub struct UnixDatagram;

impl UnixDatagram {
    pub async fn bind(_path: impl AsRef<Path>) -> Result<Self> {
        no_runtime_specified!();
    }

    pub async fn connect(&self, _path: impl AsRef<Path>) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        no_runtime_specified!();
    }

    pub async fn recv(&self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub async fn send(&self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub async fn send_to(&self, _buf: &[u8], _path: impl AsRef<Path>) -> Result<usize> {
        no_runtime_specified!();
    }

    pub fn shutdown(&self, _how: Shutdown) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn unbound() -> Result<UnixDatagram> {
        no_runtime_specified!();
    }
}

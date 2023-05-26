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

pub struct UnixDatagram(tokio::net::UnixDatagram);

impl UnixDatagram {
    pub async fn bind(path: impl AsRef<Path>) -> Result<Self> {
        tokio::net::UnixDatagram::bind(path).map(UnixDatagram)
    }

    #[inline]
    pub async fn connect(&self, path: impl AsRef<Path>) -> Result<()> {
        self.0.connect(path)
    }

    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        let (a, b) = tokio::net::UnixDatagram::pair()?;
        Ok((UnixDatagram(a), UnixDatagram(b)))
    }

    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf).await
    }

    #[inline]
    pub async fn send(&self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf).await
    }

    #[inline]
    pub async fn send_to(&self, buf: &[u8], path: impl AsRef<Path>) -> Result<usize> {
        self.0.send_to(buf, path).await
    }

    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> Result<()> {
        self.0.shutdown(how)
    }

    #[inline]
    pub fn unbound() -> Result<UnixDatagram> {
        tokio::net::UnixDatagram::unbound().map(UnixDatagram)
    }
}

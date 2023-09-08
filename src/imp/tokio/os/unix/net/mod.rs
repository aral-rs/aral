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

use crate::io::{Read, Write};
use std::{io::Result, net::Shutdown, path::Path};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct SocketAddr(tokio::net::unix::SocketAddr);

impl SocketAddr {
    #[inline]
    pub fn as_pathname(&self) -> Option<&Path> {
        self.0.as_pathname()
    }

    #[inline]
    pub fn is_unnamed(&self) -> bool {
        self.0.is_unnamed()
    }
}

pub struct UnixDatagram(tokio::net::UnixDatagram);

impl UnixDatagram {
    pub async fn bind(path: impl AsRef<Path>) -> Result<Self> {
        tokio::net::UnixDatagram::bind(path).map(UnixDatagram)
    }

    #[inline]
    pub async fn connect(&self, path: impl AsRef<Path>) -> Result<()> {
        self.0.connect(path)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        let (a, b) = tokio::net::UnixDatagram::pair()?;
        Ok((UnixDatagram(a), UnixDatagram(b)))
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr().map(SocketAddr)
    }

    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf).await
    }

    #[inline]
    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.0
            .recv_from(buf)
            .await
            .map(|(n, addr)| (n, SocketAddr(addr)))
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

pub struct UnixStream(tokio::net::UnixStream);

impl UnixStream {
    pub async fn connect(path: impl AsRef<Path>) -> Result<UnixStream> {
        tokio::net::UnixStream::connect(path).await.map(UnixStream)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    pub fn pair() -> Result<(UnixStream, UnixStream)> {
        tokio::net::UnixStream::pair().map(|(a, b)| (UnixStream(a), UnixStream(b)))
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr().map(SocketAddr)
    }
}

impl Read for UnixStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        AsyncReadExt::read(&mut self.0, buf).await
    }
}

impl Write for UnixStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        AsyncWriteExt::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        AsyncWriteExt::flush(&mut self.0).await
    }
}

pub struct UnixListener(tokio::net::UnixListener);

impl UnixListener {
    pub async fn accept(&self) -> Result<(UnixStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (UnixStream(stream), SocketAddr(addr)))
    }

    pub async fn bind(path: impl AsRef<Path>) -> Result<UnixListener> {
        tokio::net::UnixListener::bind(path).map(UnixListener)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }
}

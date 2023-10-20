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

use crate::{
    imp::{
        self,
        os::unix::net::{
            UnixDatagram as ImpUnixDatagram, UnixListener as ImpUnixListener,
            UnixStream as ImpUnixStream,
        },
    },
    io::{Read, Write},
};
use std::{io::Result, net::Shutdown, path::Path};

pub struct SocketAddr(imp::os::unix::net::SocketAddr);

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

pub struct UnixDatagram(ImpUnixDatagram);

impl UnixDatagram {
    #[inline]
    pub async fn bind(path: impl AsRef<Path>) -> Result<Self> {
        ImpUnixDatagram::bind(path).await.map(UnixDatagram)
    }

    #[inline]
    pub async fn connect(&self, path: impl AsRef<Path>) -> Result<()> {
        self.0.connect(path).await
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    #[inline]
    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        ImpUnixDatagram::pair().map(|(a, b)| (UnixDatagram(a), UnixDatagram(b)))
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
        ImpUnixDatagram::unbound().map(UnixDatagram)
    }
}

pub struct UnixStream(ImpUnixStream);

impl UnixStream {
    #[inline]
    pub async fn connect(path: impl AsRef<Path>) -> Result<UnixStream> {
        ImpUnixStream::connect(path).await.map(UnixStream)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    #[inline]
    pub fn pair() -> Result<(UnixStream, UnixStream)> {
        ImpUnixStream::pair().map(|(a, b)| (UnixStream(a), UnixStream(b)))
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr().map(SocketAddr)
    }
}

impl Read for UnixStream {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        imp::io::Read::read(&mut self.0, buf).await
    }
}

impl Write for UnixStream {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        imp::io::Write::write(&mut self.0, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<()> {
        imp::io::Write::flush(&mut self.0).await
    }
}

pub struct UnixListener(ImpUnixListener);

impl UnixListener {
    #[inline]
    pub async fn accept(&self) -> Result<(UnixStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (UnixStream(stream), SocketAddr(addr)))
    }

    #[inline]
    pub async fn bind(path: impl AsRef<Path>) -> Result<UnixListener> {
        ImpUnixListener::bind(path).await.map(UnixListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }
}

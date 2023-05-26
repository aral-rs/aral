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
    io::{Read, Write},
    net::ToSocketAddrs,
};
use std::{io::Result, net::SocketAddr};

pub struct TcpStream(async_std::net::TcpStream);

impl TcpStream {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        async_std::net::TcpStream::connect(&*addrs)
            .await
            .map(TcpStream)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }

    #[inline]
    pub fn nodelay(&self) -> Result<bool> {
        self.0.nodelay()
    }

    #[inline]
    pub async fn peek(&self, buf: &mut [u8]) -> Result<usize> {
        self.0.peek(buf).await
    }

    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> Result<()> {
        self.0.set_nodelay(nodelay)
    }

    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.0.set_ttl(ttl)
    }

    #[inline]
    pub fn ttl(&self) -> Result<u32> {
        self.0.ttl()
    }
}

impl Read for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        async_std::io::ReadExt::read(&mut self.0, buf).await
    }
}

impl Write for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        async_std::io::WriteExt::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        async_std::io::WriteExt::flush(&mut self.0).await
    }
}

pub struct TcpListener(async_std::net::TcpListener);

impl TcpListener {
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (TcpStream(stream), addr))
    }

    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        async_std::net::TcpListener::bind(&*addrs)
            .await
            .map(TcpListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

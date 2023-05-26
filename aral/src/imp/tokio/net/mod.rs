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
use std::{
    io::Result,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
};

pub struct TcpStream(tokio::net::TcpStream);

impl TcpStream {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        tokio::net::TcpStream::connect(&*addrs).await.map(TcpStream)
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
        tokio::io::AsyncReadExt::read(&mut self.0, buf).await
    }
}

impl Write for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        tokio::io::AsyncWriteExt::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        tokio::io::AsyncWriteExt::flush(&mut self.0).await
    }
}

pub struct TcpListener(tokio::net::TcpListener);

impl TcpListener {
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (TcpStream(stream), addr))
    }

    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        tokio::net::TcpListener::bind(&*addrs)
            .await
            .map(TcpListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

pub struct UdpSocket(tokio::net::UdpSocket);

impl UdpSocket {
    pub async fn bind(addr: impl ToSocketAddrs) -> Result<UdpSocket> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        tokio::net::UdpSocket::bind(&*addrs).await.map(UdpSocket)
    }

    #[inline]
    pub fn broadcast(&self) -> Result<bool> {
        self.0.broadcast()
    }

    pub async fn connect(&self, addr: impl ToSocketAddrs) -> Result<()> {
        let addrs = addr.to_socket_addrs().await?.collect::<Vec<_>>();
        self.0.connect(&*addrs).await
    }

    #[inline]
    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<()> {
        self.0.join_multicast_v4(*multiaddr, *interface)
    }

    #[inline]
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()> {
        self.0.join_multicast_v6(multiaddr, interface)
    }

    #[inline]
    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> Result<()> {
        self.0.leave_multicast_v4(*multiaddr, *interface)
    }

    #[inline]
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()> {
        self.0.leave_multicast_v6(multiaddr, interface)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    pub fn multicast_loop_v4(&self) -> Result<bool> {
        self.0.multicast_loop_v4()
    }

    #[inline]
    pub fn multicast_loop_v6(&self) -> Result<bool> {
        self.0.multicast_loop_v6()
    }

    #[inline]
    pub fn multicast_ttl_v4(&self) -> Result<u32> {
        self.0.multicast_ttl_v4()
    }

    #[inline]
    pub async fn peek_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.0.peek_from(buf).await
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }

    #[inline]
    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf).await
    }

    #[inline]
    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self.0.recv_from(buf).await
    }

    #[inline]
    pub async fn send(&self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf).await
    }

    pub async fn send_to(&self, buf: &[u8], target: impl ToSocketAddrs) -> Result<usize> {
        let target = target.to_socket_addrs().await?.collect::<Vec<_>>();
        self.0.send_to(buf, &*target).await
    }

    #[inline]
    pub fn set_broadcast(&self, on: bool) -> Result<()> {
        self.0.set_broadcast(on)
    }

    #[inline]
    pub fn set_multicast_loop_v4(&self, on: bool) -> Result<()> {
        self.0.set_multicast_loop_v4(on)
    }

    #[inline]
    pub fn set_multicast_loop_v6(&self, on: bool) -> Result<()> {
        self.0.set_multicast_loop_v6(on)
    }

    #[inline]
    pub fn set_multicast_ttl_v4(&self, ttl: u32) -> Result<()> {
        self.0.set_multicast_ttl_v4(ttl)
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

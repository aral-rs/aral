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
use std::{
    io::Result,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<Self::Iter>;
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for (String, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for (Ipv6Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for SocketAddr {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for str {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for String {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

impl ToSocketAddrs for SocketAddrV6 {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        no_runtime_specified!();
    }
}

pub struct TcpStream;

impl TcpStream {
    pub async fn connect(_addr: impl crate::net::ToSocketAddrs) -> Result<TcpStream> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn nodelay(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    pub async fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn ttl(&self) -> Result<u32> {
        no_runtime_specified!();
    }
}

impl Read for TcpStream {
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }
}

impl Write for TcpStream {
    async fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    async fn flush(&mut self) -> Result<()> {
        no_runtime_specified!();
    }
}

pub struct TcpListener;

impl TcpListener {
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        no_runtime_specified!();
    }

    pub async fn bind(_addr: impl crate::net::ToSocketAddrs) -> Result<Self> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }
}

pub struct UdpSocket;

impl UdpSocket {
    pub async fn bind(_addr: impl crate::net::ToSocketAddrs) -> Result<UdpSocket> {
        no_runtime_specified!();
    }

    pub fn broadcast(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    pub async fn connect(&self, _addr: impl crate::net::ToSocketAddrs) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn multicast_loop_v4(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    pub fn multicast_loop_v6(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    pub fn multicast_ttl_v4(&self) -> Result<u32> {
        no_runtime_specified!();
    }

    pub async fn peek_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub async fn recv(&self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub async fn recv_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        no_runtime_specified!();
    }

    pub async fn send(&self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub async fn send_to(
        &self, _buf: &[u8], _target: impl crate::net::ToSocketAddrs,
    ) -> Result<usize> {
        no_runtime_specified!();
    }

    pub fn set_broadcast(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_multicast_loop_v4(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_multicast_loop_v6(&self, _on: bool) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_multicast_ttl_v4(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn ttl(&self) -> Result<u32> {
        no_runtime_specified!();
    }
}

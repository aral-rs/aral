use crate::io::{Read, Write};
use std::{
    io::Result,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};
use tokio::net::lookup_host;

pub trait ToSocketAddrs: tokio::net::ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> impl std::future::Future<Output = Result<<Self as ToSocketAddrs>::Iter>> + Send;
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
    }
}

impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl ToSocketAddrs for (Ipv6Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl ToSocketAddrs for SocketAddr {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl ToSocketAddrs for str {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
    }
}

impl ToSocketAddrs for String {
    type Iter = std::vec::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::vec::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().collect::<Vec<_>>().into_iter())
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl ToSocketAddrs for SocketAddrV6 {
    type Iter = std::option::IntoIter<SocketAddr>;

    async fn to_socket_addrs(&self) -> Result<std::option::IntoIter<SocketAddr>> {
        lookup_host(self)
            .await
            .map(|it| it.into_iter().next().into_iter())
    }
}

impl<'a> ToSocketAddrs for &'a [SocketAddr] {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, SocketAddr>>;

    async fn to_socket_addrs(&self) -> Result<std::iter::Cloned<std::slice::Iter<'a, SocketAddr>>> {
        Ok(self.iter().cloned())
    }
}

pub struct TcpStream(tokio::net::TcpStream);

impl TcpStream {
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream> {
        tokio::net::TcpStream::connect(addr).await.map(TcpStream)
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
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        tokio::io::AsyncReadExt::read(&mut self.0, buf).await
    }
}

impl Write for TcpStream {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        tokio::io::AsyncWriteExt::write(&mut self.0, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<()> {
        tokio::io::AsyncWriteExt::flush(&mut self.0).await
    }
}

pub struct TcpListener(tokio::net::TcpListener);

impl TcpListener {
    #[inline]
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (TcpStream(stream), addr))
    }

    #[inline]
    pub async fn bind(addr: impl crate::net::ToSocketAddrs) -> Result<Self> {
        tokio::net::TcpListener::bind(addr).await.map(TcpListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

pub struct UdpSocket(tokio::net::UdpSocket);

impl UdpSocket {
    #[inline]
    pub async fn bind(addr: impl crate::net::ToSocketAddrs) -> Result<UdpSocket> {
        tokio::net::UdpSocket::bind(addr).await.map(UdpSocket)
    }

    #[inline]
    pub fn broadcast(&self) -> Result<bool> {
        self.0.broadcast()
    }

    #[inline]
    pub async fn connect(&self, addr: impl crate::net::ToSocketAddrs) -> Result<()> {
        self.0.connect(addr).await
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

    pub async fn send_to(
        &self, buf: &[u8], target: impl crate::net::ToSocketAddrs,
    ) -> Result<usize> {
        self.0.send_to(buf, target).await
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

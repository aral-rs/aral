use crate::{
    imp,
    io::{Read, Write},
};
use std::{io::Result, net::SocketAddr};

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    // Required method
    async fn to_socket_addrs(&self) -> Result<Self::Iter>;
}

pub struct TcpStream(imp::net::TcpStream);

impl TcpStream {
    #[inline]
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream> {
        imp::net::TcpStream::connect(addr).await.map(TcpStream)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }

    #[inline]
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr()
    }
}

impl Read for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Read::read(&mut self.0, buf).await
    }
}

impl Write for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Write::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        Write::flush(&mut self.0).await
    }
}

pub struct TcpListener(imp::net::TcpListener);

impl TcpListener {
    #[inline]
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (TcpStream(stream), addr))
    }

    #[inline]
    pub async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        imp::net::TcpListener::bind(addr).await.map(TcpListener)
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr()
    }
}

use crate::{
    io::{Read, Write},
    net::ToSocketAddrs,
};
use std::{io::Result, net::SocketAddr};

pub struct TcpStream;

impl TcpStream {
    pub async fn connect(_addr: impl ToSocketAddrs) -> Result<TcpStream> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
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

    pub async fn bind(_addr: impl ToSocketAddrs) -> Result<Self> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }
}

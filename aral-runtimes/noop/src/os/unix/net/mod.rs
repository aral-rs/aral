use crate::io::{Read, Write};
use std::{io::Result, net::Shutdown, path::Path};

pub struct SocketAddr;

impl SocketAddr {
    pub fn as_pathname(&self) -> Option<&Path> {
        no_runtime_specified!();
    }

    pub fn is_unnamed(&self) -> bool {
        no_runtime_specified!();
    }
}

pub struct UnixDatagram;

impl UnixDatagram {
    pub async fn bind(_path: impl AsRef<Path>) -> Result<Self> {
        no_runtime_specified!();
    }

    pub async fn connect(&self, _path: impl AsRef<Path>) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        no_runtime_specified!();
    }

    pub async fn recv(&self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub async fn recv_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
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

pub struct UnixStream;

impl UnixStream {
    pub async fn connect(_path: impl AsRef<Path>) -> Result<UnixStream> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn pair() -> Result<(UnixStream, UnixStream)> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }
}

impl Read for UnixStream {
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }
}

impl Write for UnixStream {
    async fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    async fn flush(&mut self) -> Result<()> {
        no_runtime_specified!();
    }
}

pub struct UnixListener;

impl UnixListener {
    pub async fn accept(&self) -> Result<(UnixStream, SocketAddr)> {
        no_runtime_specified!();
    }

    pub async fn bind(_path: impl AsRef<Path>) -> Result<UnixListener> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }
}

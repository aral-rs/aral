use crate::io::{Read, Write};
use async_std::{
    io::{ReadExt, WriteExt},
    os::unix::net::{
        UnixDatagram as AsyncStdUnixDatagram, UnixListener as AsyncStdUnixListener,
        UnixStream as AsyncStdUnixStream,
    },
};
use std::{io::Result, net::Shutdown, path::Path};

pub struct SocketAddr(async_std::os::unix::net::SocketAddr);

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

pub struct UnixDatagram(AsyncStdUnixDatagram);

impl UnixDatagram {
    pub async fn bind(path: impl AsRef<Path>) -> Result<Self> {
        AsyncStdUnixDatagram::bind(path.as_ref())
            .await
            .map(UnixDatagram)
    }

    #[inline]
    pub async fn connect(&self, path: impl AsRef<Path>) -> Result<()> {
        self.0.connect(path.as_ref()).await
    }

    #[inline]
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    pub fn pair() -> Result<(UnixDatagram, UnixDatagram)> {
        AsyncStdUnixDatagram::pair().map(|(a, b)| (UnixDatagram(a), UnixDatagram(b)))
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
        self.0.send_to(buf, path.as_ref()).await
    }

    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> Result<()> {
        self.0.shutdown(how)
    }

    #[inline]
    pub fn unbound() -> Result<UnixDatagram> {
        AsyncStdUnixDatagram::unbound().map(UnixDatagram)
    }
}

pub struct UnixStream(AsyncStdUnixStream);

impl UnixStream {
    pub async fn connect(path: impl AsRef<Path>) -> Result<UnixStream> {
        AsyncStdUnixStream::connect(path.as_ref())
            .await
            .map(UnixStream)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }

    pub fn pair() -> Result<(UnixStream, UnixStream)> {
        AsyncStdUnixStream::pair().map(|(a, b)| (UnixStream(a), UnixStream(b)))
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr().map(SocketAddr)
    }
}

impl Read for UnixStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        ReadExt::read(&mut self.0, buf).await
    }
}

impl Write for UnixStream {
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        WriteExt::write(&mut self.0, buf).await
    }

    async fn flush(&mut self) -> Result<()> {
        WriteExt::flush(&mut self.0).await
    }
}

pub struct UnixListener(AsyncStdUnixListener);

impl UnixListener {
    pub async fn accept(&self) -> Result<(UnixStream, SocketAddr)> {
        self.0
            .accept()
            .await
            .map(|(stream, addr)| (UnixStream(stream), SocketAddr(addr)))
    }

    pub async fn bind(path: impl AsRef<Path>) -> Result<UnixListener> {
        AsyncStdUnixListener::bind(path.as_ref())
            .await
            .map(UnixListener)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(SocketAddr)
    }
}

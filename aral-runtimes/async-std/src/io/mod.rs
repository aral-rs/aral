use futures_lite::io::{AsyncBufReadExt, AsyncReadExt};
use std::{
    future::Future,
    io::{Result, SeekFrom},
};

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> + Send;
}

impl<T: ?Sized + Read + Unpin + Send> Read for &mut T {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Read::read(&mut **self, buf).await
    }
}

impl<T: ?Sized + Read + Unpin + Send> Read for Box<T> {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Read::read(&mut **self, buf).await
    }
}

pub trait BufRead: Read {
    fn fill_buf(&mut self) -> impl Future<Output = Result<&[u8]>> + Send;

    fn consume(&mut self, amt: usize);
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize>> + Send;

    fn flush(&mut self) -> impl Future<Output = Result<()>> + Send;
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> impl Future<Output = Result<u64>> + Send;
}

#[repr(transparent)]
pub struct Empty(async_std::io::Empty);

impl Read for Empty {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf).await
    }
}

impl BufRead for Empty {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8]> {
        self.0.fill_buf().await
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

#[inline]
pub fn empty() -> Empty {
    Empty(async_std::io::empty())
}

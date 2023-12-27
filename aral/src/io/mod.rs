use crate::imp;
use std::{
    future::Future,
    io::{Result, SeekFrom},
};

pub trait Read: imp::io::Read {
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

pub trait BufRead: Read + imp::io::Read + imp::io::BufRead {
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
pub struct Empty(imp::io::Empty);

impl Read for Empty {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        imp::io::Read::read(&mut self.0, buf).await
    }
}

impl BufRead for Empty {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8]> {
        imp::io::BufRead::fill_buf(&mut self.0).await
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        imp::io::BufRead::consume(&mut self.0, amt)
    }
}

impl imp::io::Read for Empty {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        imp::io::Read::read(&mut self.0, buf).await
    }
}

impl imp::io::BufRead for Empty {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8]> {
        imp::io::BufRead::fill_buf(&mut self.0).await
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        imp::io::BufRead::consume(&mut self.0, amt)
    }
}

#[inline]
pub fn empty() -> Empty {
    Empty(imp::io::empty())
}

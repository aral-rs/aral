use std::{
    future::Future,
    io::{Result, SeekFrom},
};

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> + Send;
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

pub struct Empty;

impl Read for Empty {
    #[inline]
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }
}

impl BufRead for Empty {
    #[inline]
    async fn fill_buf(&mut self) -> Result<&[u8]> {
        no_runtime_specified!();
    }

    #[inline]
    fn consume(&mut self, _amt: usize) {
        no_runtime_specified!();
    }
}

#[inline]
pub fn empty() -> Empty {
    no_runtime_specified!();
}

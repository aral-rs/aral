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

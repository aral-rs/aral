use std::io::{Result, SeekFrom};

pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait BufRead: Read {
    async fn fill_buf(&mut self) -> Result<&[u8]>;

    async fn consume(&mut self, amt: usize);
}

pub trait Write {
    async fn write(&mut self, buf: &[u8]) -> Result<usize>;

    async fn flush(&mut self) -> Result<()>;
}

pub trait Seek {
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

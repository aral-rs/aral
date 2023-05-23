use std::io::SeekFrom;

pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}

pub trait BufRead: Read {
    async fn fill_buf(&mut self) -> std::io::Result<&[u8]>;

    async fn consume(&mut self, amt: usize);
}

pub trait Write {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;

    async fn flush(&mut self) -> std::io::Result<()>;
}

pub trait Seek {
    async fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64>;
}

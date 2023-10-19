use crate::io::{Read, Seek, Write};
use std::{
    fs::{Metadata, Permissions},
    io::{Result, SeekFrom},
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub struct File(tokio::fs::File);

impl File {
    #[inline]
    pub async fn create(path: impl AsRef<Path>) -> Result<File> {
        tokio::fs::File::create(path).await.map(File)
    }

    #[inline]
    pub async fn open(path: impl AsRef<Path>) -> Result<File> {
        tokio::fs::File::open(path).await.map(File)
    }

    #[inline]
    pub async fn metadata(&self) -> Result<Metadata> {
        self.0.metadata().await
    }

    #[inline]
    pub async fn set_len(&self, size: u64) -> Result<()> {
        self.0.set_len(size).await
    }

    #[inline]
    pub async fn set_permissions(&self, perm: Permissions) -> Result<()> {
        self.0.set_permissions(perm).await
    }

    #[inline]
    pub async fn sync_all(&self) -> Result<()> {
        self.0.sync_all().await
    }

    #[inline]
    pub async fn sync_data(&self) -> Result<()> {
        self.0.sync_data().await
    }

    #[inline]
    pub async fn try_clone(&self) -> Result<File> {
        self.0.try_clone().await.map(File)
    }
}

impl Read for File {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        AsyncReadExt::read(&mut self.0, buf).await
    }
}

impl Write for File {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        AsyncWriteExt::write(&mut self.0, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<()> {
        AsyncWriteExt::flush(&mut self.0).await
    }
}

impl Seek for File {
    #[inline]
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        AsyncSeekExt::seek(&mut self.0, pos).await
    }
}

pub struct OpenOptions(tokio::fs::OpenOptions);

impl OpenOptions {
    #[inline]
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    #[inline]
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.0.create(create);
        self
    }

    #[inline]
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.0.create_new(create_new);
        self
    }

    #[inline]
    pub fn new() -> OpenOptions {
        OpenOptions(tokio::fs::OpenOptions::new())
    }

    #[inline]
    pub async fn open(&self, path: impl AsRef<Path>) -> Result<File> {
        self.0.open(path).await.map(File)
    }

    #[inline]
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    #[inline]
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.0.truncate(truncate);
        self
    }

    #[inline]
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }
}

#[inline]
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    tokio::fs::canonicalize(path).await
}

#[inline]
pub async fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    tokio::fs::copy(from, to).await
}

#[inline]
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::create_dir(path).await
}

#[inline]
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::create_dir_all(path).await
}

#[inline]
pub async fn hard_link(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    tokio::fs::hard_link(src, dst).await
}

#[inline]
pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    tokio::fs::metadata(path).await
}

#[inline]
pub async fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    tokio::fs::read(path).await
}

#[inline]
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf> {
    tokio::fs::read_link(path).await
}

#[inline]
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    tokio::fs::read_to_string(path).await
}

#[inline]
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::remove_dir(path).await
}

#[inline]
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::remove_dir_all(path).await
}

#[inline]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::remove_file(path).await
}

#[inline]
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    tokio::fs::rename(from, to).await
}

#[inline]
pub async fn set_permissions(path: impl AsRef<Path>, perm: Permissions) -> Result<()> {
    tokio::fs::set_permissions(path, perm).await
}

#[inline]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    tokio::fs::symlink_metadata(path).await
}

#[inline]
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    tokio::fs::write(path, contents).await
}

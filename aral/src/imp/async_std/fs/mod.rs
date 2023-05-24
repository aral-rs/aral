use crate::io::{Read, Seek, Write};
use std::{
    fs::{Metadata, Permissions},
    io::{Result, SeekFrom},
    path::{Path, PathBuf},
};

pub struct File(async_std::fs::File);

impl File {
    #[inline]
    pub async fn create(path: impl AsRef<Path>) -> Result<File> {
        async_std::fs::File::create(path.as_ref()).await.map(File)
    }

    #[inline]
    pub async fn open(path: impl AsRef<Path>) -> Result<File> {
        async_std::fs::File::open(path.as_ref()).await.map(File)
    }
}

impl Read for File {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        async_std::io::ReadExt::read(&mut self.0, buf).await
    }
}

impl Write for File {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        async_std::io::WriteExt::write(&mut self.0, buf).await
    }

    #[inline]
    async fn flush(&mut self) -> Result<()> {
        async_std::io::WriteExt::flush(&mut self.0).await
    }
}

impl Seek for File {
    #[inline]
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        async_std::io::prelude::SeekExt::seek(&mut self.0, pos).await
    }
}

#[inline]
pub async fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
    async_std::fs::canonicalize(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64> {
    async_std::fs::copy(from.as_ref(), to.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    async_std::fs::create_dir(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    async_std::fs::create_dir_all(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn hard_link(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    async_std::fs::hard_link(src.as_ref(), dst.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    async_std::fs::metadata(path.as_ref()).await.map(Into::into)
}

#[inline]
pub async fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    async_std::fs::read(path.as_ref()).await.map(Into::into)
}

#[inline]
pub async fn read_link(path: impl AsRef<Path>) -> Result<PathBuf> {
    async_std::fs::read_link(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    async_std::fs::read_to_string(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    async_std::fs::remove_dir(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    async_std::fs::remove_dir_all(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    async_std::fs::remove_file(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    async_std::fs::rename(from.as_ref(), to.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn set_permissions(path: impl AsRef<Path>, perm: Permissions) -> Result<()> {
    async_std::fs::set_permissions(path.as_ref(), perm)
        .await
        .map(Into::into)
}

#[inline]
pub async fn symlink_metadata(path: impl AsRef<Path>) -> Result<Metadata> {
    async_std::fs::symlink_metadata(path.as_ref())
        .await
        .map(Into::into)
}

#[inline]
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    async_std::fs::write(path.as_ref(), contents)
        .await
        .map(Into::into)
}

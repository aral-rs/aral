use std::{
    fs::{Metadata, Permissions},
    io::Result,
    path::{Path, PathBuf},
};

pub struct File(tokio::fs::File);

impl File {
    #[inline]
    pub async fn create(path: impl AsRef<Path>) -> Result<File> {
        Ok(File(tokio::fs::File::create(path).await?))
    }

    #[inline]
    pub async fn open(path: impl AsRef<Path>) -> Result<File> {
        Ok(File(tokio::fs::File::open(path).await?))
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

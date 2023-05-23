use std::{
    fs::{Metadata, Permissions},
    io::Result,
    path::{Path, PathBuf},
};

pub struct File;

impl File {
    #[inline]
    pub async fn create(_path: impl AsRef<Path>) -> Result<File> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn open(_path: impl AsRef<Path>) -> Result<File> {
        no_runtime_specified!();
    }
}

#[inline]
pub async fn canonicalize(_path: impl AsRef<Path>) -> Result<PathBuf> {
    no_runtime_specified!();
}

#[inline]
pub async fn copy(_from: impl AsRef<Path>, _to: impl AsRef<Path>) -> Result<u64> {
    no_runtime_specified!();
}

#[inline]
pub async fn create_dir(_path: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn create_dir_all(_path: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn hard_link(_src: impl AsRef<Path>, _dst: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn metadata(_path: impl AsRef<Path>) -> Result<Metadata> {
    no_runtime_specified!();
}

#[inline]
pub async fn read(_path: impl AsRef<Path>) -> Result<Vec<u8>> {
    no_runtime_specified!();
}

#[inline]
pub async fn read_link(_path: impl AsRef<Path>) -> Result<PathBuf> {
    no_runtime_specified!();
}

#[inline]
pub async fn read_to_string(_path: impl AsRef<Path>) -> Result<String> {
    no_runtime_specified!();
}

#[inline]
pub async fn remove_dir(_path: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn remove_dir_all(_path: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn remove_file(_path: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn rename(_from: impl AsRef<Path>, _to: impl AsRef<Path>) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn set_permissions(_path: impl AsRef<Path>, _perm: Permissions) -> Result<()> {
    no_runtime_specified!();
}

#[inline]
pub async fn symlink_metadata(_path: impl AsRef<Path>) -> Result<Metadata> {
    no_runtime_specified!();
}

#[inline]
pub async fn write(_path: impl AsRef<Path>, _contents: impl AsRef<[u8]>) -> Result<()> {
    no_runtime_specified!();
}

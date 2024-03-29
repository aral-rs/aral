use crate::io::{Read, Seek, Write};
use std::{
    fs::{Metadata, Permissions},
    io::{Result, SeekFrom},
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

    #[inline]
    pub async fn metadata(&self) -> Result<Metadata> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn set_len(&self, _size: u64) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn set_permissions(&self, _perm: Permissions) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn sync_all(&self) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn sync_data(&self) -> Result<()> {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn try_clone(&self) -> Result<File> {
        no_runtime_specified!();
    }
}

impl Read for File {
    #[inline]
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }
}

impl Write for File {
    #[inline]
    async fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    #[inline]
    async fn flush(&mut self) -> Result<()> {
        no_runtime_specified!();
    }
}

impl Seek for File {
    #[inline]
    async fn seek(&mut self, _pos: SeekFrom) -> Result<u64> {
        no_runtime_specified!();
    }
}

pub struct OpenOptions;

impl OpenOptions {
    #[inline]
    pub fn append(&mut self, _append: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub fn create(&mut self, _create: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub fn create_new(&mut self, _create_new: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub fn new() -> OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub async fn open(&self, _path: impl AsRef<Path>) -> Result<File> {
        no_runtime_specified!();
    }

    #[inline]
    pub fn read(&mut self, _read: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub fn truncate(&mut self, _truncate: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }

    #[inline]
    pub fn write(&mut self, _write: bool) -> &mut OpenOptions {
        no_runtime_specified!();
    }
}

impl Default for OpenOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
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

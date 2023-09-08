// Copyright 2023 ARAL Development Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        Ok(File(self.0.clone()))
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

pub struct OpenOptions(async_std::fs::OpenOptions);

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
        OpenOptions(async_std::fs::OpenOptions::new())
    }

    #[inline]
    pub async fn open(&self, path: impl AsRef<Path>) -> Result<File> {
        self.0.open(path.as_ref()).await.map(File)
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

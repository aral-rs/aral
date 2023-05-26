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

use std::io::{Result, SeekFrom};

pub trait Read {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait BufRead: Read {
    async fn fill_buf(&mut self) -> Result<&[u8]>;

    fn consume(&mut self, amt: usize);
}

pub trait Write {
    async fn write(&mut self, buf: &[u8]) -> Result<usize>;

    async fn flush(&mut self) -> Result<()>;
}

pub trait Seek {
    async fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

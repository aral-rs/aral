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

use crate::{
    io::{Read, Write},
    net::ToSocketAddrs,
};
use std::{io::Result, net::SocketAddr};

pub struct TcpStream;

impl TcpStream {
    pub async fn connect(_addr: impl ToSocketAddrs) -> Result<TcpStream> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }

    pub fn nodelay(&self) -> Result<bool> {
        no_runtime_specified!();
    }

    pub async fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn set_ttl(&self, _ttl: u32) -> Result<()> {
        no_runtime_specified!();
    }

    pub fn ttl(&self) -> Result<u32> {
        no_runtime_specified!();
    }
}

impl Read for TcpStream {
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        no_runtime_specified!();
    }
}

impl Write for TcpStream {
    async fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        no_runtime_specified!();
    }

    async fn flush(&mut self) -> Result<()> {
        no_runtime_specified!();
    }
}

pub struct TcpListener;

impl TcpListener {
    pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        no_runtime_specified!();
    }

    pub async fn bind(_addr: impl ToSocketAddrs) -> Result<Self> {
        no_runtime_specified!();
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        no_runtime_specified!();
    }
}

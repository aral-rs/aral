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

#![feature(async_fn_in_trait)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docs_rs, feature(doc_cfg))]

mod exp;
mod imp;

pub use crate::{
    exp::*,
    imp::{current_runtime_type, RuntimeType},
};

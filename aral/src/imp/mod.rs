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

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(feature = "runtime-tokio-multi-thread", feature = "runtime-tokio-current-thread"))] {
        mod tokio;
        pub(crate) use self::tokio::*;

    } else if #[cfg(feature = "runtime-async-std")] {
        mod async_std;
        pub(crate) use self::async_std::*;

    } else {
        mod no_runtime;
        pub(crate) use self::no_runtime::*;
    }

}

#[non_exhaustive]
pub enum RuntimeType {
    NoRuntime,
    TokioMultiThread,
    TokioCurrentThread,
    AsyncStd,
}

pub const fn current_runtime_type() -> RuntimeType {
    if cfg!(feature = "runtime-tokio-multi-thread") {
        return RuntimeType::TokioMultiThread;
    }

    if cfg!(feature = "runtime-tokio-current-thread") {
        return RuntimeType::TokioCurrentThread;
    }

    if cfg!(feature = "runtime-async-std") {
        return RuntimeType::AsyncStd;
    }

    RuntimeType::NoRuntime
}

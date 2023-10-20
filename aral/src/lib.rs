#![doc = include_str!("../README.md")]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

pub mod fs;
pub mod io;
pub mod net;
pub mod os;
pub mod task;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "runtime-tokio"))] {
        use aral_runtime_tokio as imp;

    } else if #[cfg(feature = "runtime-async-std")] {
        use aral_runtime_async_std as imp;

    } else if #[cfg(feature = "runtime-noop")] {
        use aral_runtime_noop as imp;

    } else {
        compile_error!("no runtime specified");
    }
}

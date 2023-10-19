#![doc = include_str!("../../README.md")]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

pub mod fs;
pub mod io;
pub mod net;
pub mod os;
pub mod task;
mod noop;

cfg_if::cfg_if! {
    if #[cfg(any(feature = "runtime-tokio"))] {

    } else if #[cfg(feature = "runtime-async-std")] {

    } else {
        use noop as imp;
    }
}

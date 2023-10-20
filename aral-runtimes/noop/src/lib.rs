#![doc = include_str!("../README.md")]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

macro_rules! no_runtime_specified {
    () => {
        panic!("no runtime specified, please enable one of `runtime-*` features");
    };
}

pub mod fs;
pub mod net;
pub mod os;
pub mod task;
pub mod io;

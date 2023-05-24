#![feature(async_fn_in_trait)]
#![doc = include_str!("../README.md")]

mod exp;
mod imp;

pub use crate::{
    exp::*,
    imp::{current_runtime_type, RuntimeType},
};

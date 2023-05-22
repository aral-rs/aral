#![macro_use]

#[macro_export]
macro_rules! no_runtime_specified {
    () => {
        panic!("no runtime specified, please enable one of `runtime-*` features");
    };
}

pub(crate) mod rt;
pub(crate) mod time;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "runtime-tokio")] {
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

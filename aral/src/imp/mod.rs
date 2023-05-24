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

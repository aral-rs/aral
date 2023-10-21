# ARAL - Async Runtime Aggregation Layer

[<img alt="github.com" src="https://img.shields.io/badge/github-aral-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/aral-rs/aral)
[<img alt="crates.io" src="https://img.shields.io/crates/v/aral.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/aral)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-aral-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/aral)

ARAL is an aggregation layer between your application and the executor for your async stuff.
It lets you switch the executors smooth and easy without having to change your applications code.

## Runtime

**Note**: Libraries should not enable any runtime features. You can choose the executor, by using cargo features.
There can only be one enabled runtime. Valid features are:

- **runtime-tokio**
- **runtime-async-std**

## Principle

1. NOT implement async runtime.

   Does not implement a concrete asynchronous runtime, only aggregate out-of-the-box asynchronous
   runtimes.

1. Minimum available.

   Try to be as minimal as possible, only add necessary methods, and do not add additional tools
   (such as channels).

1. Consistent with std.

   The asynchronous API style should be as consistent as possible with the synchronous API of std.

## Example

For libraries, use `aral` as a dependency and do not enable any `runtime-*` features.

```toml
# Cargo.toml

[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
aral = "*"
```

```rust, ignore
// lib.rs

use aral::fs::File;
use std::io::Result;

pub async fn is_file(path: &str) -> Result<bool> {
   let file = File::open(path).await?;
   let metadata = file.metadata().await?;
   Ok(metadata.is_file())
}
```

For application, use `aral` as a dependency and enable one `runtime-*` features.

```toml
# Cargo.toml

[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
aral = { version = "*", features = ["runtime-tokio"] }
tokio = { version = "1.33.0", features = ["full"] }
foo = "0.1.0"
```

```rust, ignore
// main.rs

use foo::is_file;

#[tokio::main]
async fn main() {
   let path = "/tmp/some-file.txt";
   println!("{} is file: {:?}", path, is_file(path).await);
}
```

Now, you can easily switch between async runtimes by enabling other `runtime-*` features.

## License

Apache-2.0 OR MIT OR MulanPSL-2.0

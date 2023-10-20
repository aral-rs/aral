# ARAL - Async Runtime Aggregation Layer

ARAL is an aggregation layer between your application and the executor for your async stuff.
It lets you switch the executors smooth and easy without having to change your applications code.

## Runtime

**Note**: Libraries should not enable any runtime feature. You can choose the executor, by using cargo features.
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

For application, use `aral` as a dependency and enable one `runtime-*` features (better to disable default features).

```toml
# Cargo.toml

[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
aral = { version = "*", features = ["runtime-tokio"], default-features = false }
tokio = { version = "1.33.0", features = ["full"] }
foo = 0.1.0
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

## License

Apache-2.0 OR MIT OR MulanPSL-2.0

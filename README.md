# ARAL - Async Runtime Aggregation Layer

ARAL is an aggregation layer between your application and the executor for your async stuff.
It lets you switch the executors smooth and easy without having to change your applications code.

## Runtime

**Note**: Libraries should not enable any runtime feature. You can choose the executor, by using cargo features.
There can only be one enabled runtime. Valid features are:

- **runtime-tokio-current-thread**
- **runtime-tokio-multi-thread**
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

## License

Apache 2.0

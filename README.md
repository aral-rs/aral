# ARAL - Async Runtime Aggregation Layer

ARAL is an aggregation layer between your application and the executor for your async stuff.
It lets you switch the executors smooth and easy without having to change your applications code.

## Runtime

**Note**: Libraries should not enable any runtime feature. You can choose the executor, by using cargo features.
There can only be one enabled runtime. Valid features are:

- **runtime-tokio-current-thread**
- **runtime-tokio-multi-thread**
- **runtime-async-std**

## License

Apache 2.0

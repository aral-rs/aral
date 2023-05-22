use std::time::Duration;

#[inline]
pub(crate) async fn sleep(_duration: Duration) {
    no_runtime_specified!();
}

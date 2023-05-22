use std::time::Duration;

#[inline]
pub(crate) async fn sleep(duration: Duration) {
    async_std::task::sleep(duration).await
}

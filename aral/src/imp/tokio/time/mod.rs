use std::time::Duration;

#[inline]
pub(crate) async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await
}

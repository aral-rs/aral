use crate::imp;
use std::time::Duration;

#[inline]
pub async fn sleep(duration: Duration) {
    imp::time::sleep(duration).await
}

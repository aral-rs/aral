use aral::{rt::Builder, time::sleep};
use std::time::Duration;

#[test]
fn test_sleep() {
    let runtime = Builder::new().build().unwrap();
    runtime.block_on(async move {
        sleep(Duration::from_secs(1)).await;
    });
}

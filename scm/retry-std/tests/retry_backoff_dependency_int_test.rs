//! The shared backoff scheduler (`edge-transport-retry`) driven by this middleware's
//! `RetryConfig`, which implements the policy crate's `BackoffPolicy` in the contract package.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use edge_transport_http_egress_retry_api::RetryConfig;
use edge_transport_retry::BackoffScheduler;

fn config() -> RetryConfig {
    RetryConfig {
        max_retries: 5,
        initial_interval_ms: 100,
        max_interval_ms: 250,
        multiplier: 2.0,
        jitter_factor: 0.0,
        ..RetryConfig::default()
    }
}

/// @covers: edge_transport_retry
#[test]
fn std_struct_backoff_scheduler_first_retry_waits_the_initial_interval_int_test() {
    let wait = BackoffScheduler::next_backoff(&config(), 0, 0.0);
    assert_eq!(wait, Duration::from_millis(100));
}

/// @covers: edge_transport_retry
#[test]
fn std_struct_backoff_scheduler_grows_by_the_multiplier_int_test() {
    let wait = BackoffScheduler::next_backoff(&config(), 1, 0.0);
    assert_eq!(wait, Duration::from_millis(200));
}

/// @covers: edge_transport_retry
#[test]
fn std_struct_backoff_scheduler_is_capped_at_the_max_interval_int_test() {
    // 100ms * 2^2 = 400ms, above the 250ms cap.
    let wait = BackoffScheduler::next_backoff(&config(), 2, 0.0);
    assert_eq!(wait, Duration::from_millis(250));
}

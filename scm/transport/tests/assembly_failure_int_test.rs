//! Integration tests for `AssemblyFailure`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::HttpTransportSvc;

fn loader(content: &str) -> (tempfile::TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = swe_edge_configbuilder::ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

fn err_message(
    result: Result<
        Box<dyn edge_transport_http_egress_transport::HttpEgress>,
        edge_transport_http_egress_transport::HttpEgressBuildError,
    >,
) -> String {
    match result {
        Ok(_) => panic!("expected an assembly failure, got Ok"),
        Err(e) => e.to_string(),
    }
}

/// A wrapped assembly failure (here: an invalid `[retry]` section reaching
/// `HttpEgressBuildError::Retry(AssemblyFailure(..))`) must preserve the
/// original error's message through `Display`, not collapse it to a generic
/// placeholder string; a config with no recognised sections, by contrast,
/// must not produce any `AssemblyFailure` at all.
#[test]
fn test_assembly_failure_display_preserves_original_message_happy() {
    let (_d, l) = loader(
        "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\nmax_interval_ms = 10000\n\
         multiplier = 0.0\nretryable_statuses = [503]\nretryable_methods = [\"GET\"]\n",
    );
    let message = err_message(HttpTransportSvc::http_egress_from_config(&l));
    assert!(
        message.contains("multiplier"),
        "AssemblyFailure's Display must surface the wrapped error's real message, got: {message}"
    );

    let (_d2, l2) = loader("[unrelated]\nx = 1");
    assert!(
        HttpTransportSvc::http_egress_from_config(&l2).is_ok(),
        "a config with no recognised sections must not produce an AssemblyFailure"
    );
}

/// Two independently invalid sections (`[retry]` vs a malformed `[cache]`)
/// must each surface their own distinct message through the same
/// `AssemblyFailure` wrapper — proving `Display` isn't hardcoded to one
/// fixed string regardless of which sibling crate's error it wraps.
#[test]
fn test_assembly_failure_display_varies_by_source_error_edge() {
    let (_d, retry_bad) = loader(
        "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\nmax_interval_ms = 10000\n\
         multiplier = 0.0\nretryable_statuses = [503]\nretryable_methods = [\"GET\"]\n",
    );
    let (_d2, cache_bad) = loader("[cache]\nbogus = 1");

    let retry_msg = err_message(HttpTransportSvc::http_egress_from_config(&retry_bad));
    let cache_msg = err_message(HttpTransportSvc::http_egress_from_config(&cache_bad));

    assert_ne!(
        retry_msg, cache_msg,
        "distinct wrapped errors must render distinct AssemblyFailure messages"
    );
}

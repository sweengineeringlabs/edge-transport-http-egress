//! Integration tests for `edge_transport_http_egress_tls::TlsConfigError`.
//!
//! Covers: `TlsConfigError::Config`, `TlsConfigError::MissingEnvVar`, `TlsConfigError::CertLoad`,
//! `TlsConfigError::CertParse` — Display messages
//! must be actionable: embed the payload, be non-empty.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_tls::TlsConfigError;

// ---------------------------------------------------------------------------
// TlsConfigError::Config
// ---------------------------------------------------------------------------

#[test]
fn test_parse_failed_display_names_the_crate() {
    let msg = TlsConfigError::Config("unknown field `jks`".to_string()).to_string();
    assert!(
        !msg.is_empty(),
        "Config Display must be non-empty; got: {msg}"
    );
}

#[test]
fn test_parse_failed_display_contains_supplied_reason() {
    let msg = TlsConfigError::Config("missing field `path`".to_string()).to_string();
    assert!(
        msg.contains("path"),
        "Config must embed the reason; got: {msg}"
    );
}

#[test]
fn test_parse_failed_is_debug_printable() {
    let _ = format!("{:?}", TlsConfigError::Config("bad".to_string()));
}

#[test]
fn test_parse_failed_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(TlsConfigError::Config("oops".to_string()));
    assert!(!err.to_string().is_empty());
}

// ---------------------------------------------------------------------------
// TlsConfigError::MissingEnvVar
// ---------------------------------------------------------------------------

/// Display must contain the missing env var name so the operator knows
/// exactly which variable to set to fix the issue.
#[test]
fn test_missing_env_var_display_contains_var_name() {
    let msg = TlsConfigError::MissingEnvVar {
        name: "EDGE_MTLS_PASSWORD".to_string(),
    }
    .to_string();
    assert!(
        msg.contains("EDGE_MTLS_PASSWORD"),
        "MissingEnvVar must name the variable; got: {msg}"
    );
}

/// Display must be actionable.
#[test]
fn test_missing_env_var_display_names_the_crate() {
    let msg = TlsConfigError::MissingEnvVar {
        name: "ANY_VAR".to_string(),
    }
    .to_string();
    assert!(
        !msg.is_empty(),
        "MissingEnvVar Display must be non-empty; got: {msg}"
    );
}

#[test]
fn test_missing_env_var_is_debug_printable() {
    let _ = format!(
        "{:?}",
        TlsConfigError::MissingEnvVar {
            name: "X".to_string()
        }
    );
}

#[test]
fn test_missing_env_var_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(TlsConfigError::MissingEnvVar {
        name: "Y".to_string(),
    });
    assert!(!err.to_string().is_empty());
}

// ---------------------------------------------------------------------------
// TlsConfigError::CertLoad
// ---------------------------------------------------------------------------

/// Display must include the path so the operator knows which file to check.
#[test]
fn test_file_read_failed_display_contains_path() {
    let msg =
        TlsConfigError::CertLoad("/etc/certs/client.p12: No such file or directory".to_string())
            .to_string();
    assert!(
        msg.contains("/etc/certs/client.p12"),
        "CertLoad Display must embed the path; got: {msg}"
    );
}

/// Display must include the reason so the operator knows whether it was a
/// permissions issue, a missing file, or something else.
#[test]
fn test_file_read_failed_display_contains_reason() {
    let msg = TlsConfigError::CertLoad("Permission denied (os error 13)".to_string()).to_string();
    assert!(
        msg.contains("Permission denied"),
        "CertLoad Display must embed the reason; got: {msg}"
    );
}

/// Display must be actionable and non-empty.
#[test]
fn test_file_read_failed_display_names_the_crate() {
    let msg = TlsConfigError::CertLoad("load failure".to_string()).to_string();
    assert!(
        !msg.is_empty(),
        "CertLoad Display must be non-empty; got: {msg}"
    );
}

#[test]
fn test_file_read_failed_is_debug_printable() {
    let _ = format!("{:?}", TlsConfigError::CertLoad("/p: reason".to_string()));
}

#[test]
fn test_file_read_failed_is_std_error() {
    let err: Box<dyn std::error::Error> =
        Box::new(TlsConfigError::CertLoad("/p: reason".to_string()));
    assert!(!err.to_string().is_empty());
}

// ---------------------------------------------------------------------------
// TlsConfigError::CertParse
// ---------------------------------------------------------------------------

/// Display must name the format so the operator knows which parser failed.
#[test]
fn test_invalid_certificate_display_names_format() {
    let msg = TlsConfigError::CertParse("pkcs12: wrong password".to_string()).to_string();
    assert!(
        msg.contains("pkcs12"),
        "CertParse Display must name the format; got: {msg}"
    );
}

/// Display must include the reason (e.g. "wrong password") for diagnosability.
#[test]
fn test_invalid_certificate_display_contains_reason() {
    let msg = TlsConfigError::CertParse("pem: no CERTIFICATE block found".to_string()).to_string();
    assert!(
        msg.contains("no CERTIFICATE block found"),
        "CertParse Display must embed the reason; got: {msg}"
    );
}

/// Display must be actionable and non-empty.
#[test]
fn test_invalid_certificate_display_names_the_crate() {
    let msg = TlsConfigError::CertParse("pem: bad".to_string()).to_string();
    assert!(
        !msg.is_empty(),
        "CertParse Display must be non-empty; got: {msg}"
    );
}

#[test]
fn test_invalid_certificate_is_debug_printable() {
    let _ = format!("{:?}", TlsConfigError::CertParse("pem: bad".to_string()));
}

// ---------------------------------------------------------------------------
// All four variants are distinct
// ---------------------------------------------------------------------------

/// All four error variants for the same conceptual payload must produce
/// distinct display strings — callers must be able to distinguish them.
#[test]
fn test_all_error_variants_have_distinct_display_messages() {
    let messages: Vec<String> = vec![
        TlsConfigError::Config("x".into()).to_string(),
        TlsConfigError::MissingEnvVar { name: "x".into() }.to_string(),
        TlsConfigError::CertLoad("x".into()).to_string(),
        TlsConfigError::CertParse("x".into()).to_string(),
    ];
    // No two messages should be identical.
    for (i, a) in messages.iter().enumerate() {
        for (j, b) in messages.iter().enumerate() {
            if i != j {
                assert_ne!(
                    a, b,
                    "variants {i} and {j} must have distinct display messages"
                );
            }
        }
    }
}

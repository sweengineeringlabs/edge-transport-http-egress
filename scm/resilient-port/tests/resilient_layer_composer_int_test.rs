#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Consumer-facing integration checks for the resilient composition contract.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use configbuilder::ConfigLoaderFactory;
use edge_transport_http_egress_cassette::CassetteConfig;
use edge_transport_http_egress_resilient_port::{
    ResilientConfigRequest, ResilientDefaultsRequest, ResilientError, ResilientLayerComposer,
};
use reqwest_middleware::ClientBuilder;

struct PassThroughLayers;

impl ResilientLayerComposer for PassThroughLayers {
    fn apply_from_config(
        &self,
        request: ResilientConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError> {
        Ok(request.builder)
    }

    fn apply_defaults(
        &self,
        request: ResilientDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError> {
        Ok(request.builder)
    }
}

struct FailingLayers;

impl ResilientLayerComposer for FailingLayers {
    fn apply_from_config(
        &self,
        _request: ResilientConfigRequest<'_>,
    ) -> Result<ClientBuilder, ResilientError> {
        Err(ResilientError::Config("invalid configuration".into()))
    }

    fn apply_defaults(
        &self,
        _request: ResilientDefaultsRequest,
    ) -> Result<ClientBuilder, ResilientError> {
        Err(ResilientError::Cassette("invalid cassette".into()))
    }
}

fn builder() -> ClientBuilder {
    ClientBuilder::new(reqwest::Client::new())
}

fn builder_with_caller_header() -> ClientBuilder {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "x-caller",
        reqwest::header::HeaderValue::from_static("preserved"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("build client with caller defaults");
    ClientBuilder::new(client)
}

fn one_response_server(expected_header: Option<&'static [u8]>) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local test server");
    let address = listener.local_addr().expect("read local server address");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept one client");
        respond(&mut stream, expected_header);
    });
    (format!("http://{address}/health"), server)
}

fn respond(stream: &mut TcpStream, expected_header: Option<&[u8]>) {
    let mut request = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !request.windows(4).any(|window| window == b"\r\n\r\n") && request.len() < 8192 {
        let count = stream.read(&mut chunk).expect("read request bytes");
        if count == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..count]);
    }

    let caller_header_present = expected_header.is_none_or(|header| {
        request
            .windows(header.len())
            .any(|window| window.eq_ignore_ascii_case(header))
    });
    let (status, body) = if caller_header_present {
        ("200 OK", b"ok".as_slice())
    } else {
        ("400 Bad Request", b"missing caller header".as_slice())
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(response.as_bytes())
        .expect("write headers");
    stream.write_all(body).expect("write response");
}

async fn assert_usable_client(builder: ClientBuilder, expected_header: Option<&'static [u8]>) {
    let client = builder.build();
    let (url, server) = one_response_server(expected_header);
    let response = client.get(url).send().await.expect("send HTTP request");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.text().await.expect("read response body"), "ok");
    server.join().expect("server thread completes");
}

/// @covers: apply_from_config
#[tokio::test]
async fn test_apply_from_config_pass_through_happy() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(PassThroughLayers);
    let loader = ConfigLoaderFactory::create_loader().expect("create config loader");
    let client_builder = layers
        .apply_from_config(ResilientConfigRequest {
            builder: builder(),
            loader: &loader,
        })
        .expect("apply config path");
    assert_usable_client(client_builder, None).await;
}

/// @covers: apply_from_config
#[tokio::test]
async fn test_apply_from_config_preserves_client_defaults_edge() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(PassThroughLayers);
    let loader = ConfigLoaderFactory::create_loader().expect("create config loader");
    let client_builder = layers
        .apply_from_config(ResilientConfigRequest {
            builder: builder_with_caller_header(),
            loader: &loader,
        })
        .expect("apply config path");
    assert_usable_client(client_builder, Some(b"x-caller: preserved")).await;
}

/// @covers: apply_defaults
#[tokio::test]
async fn test_apply_defaults_pass_through_happy() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(PassThroughLayers);
    let client_builder = layers
        .apply_defaults(ResilientDefaultsRequest {
            builder: builder(),
            cassette: CassetteConfig::disabled(),
            cassette_name: "integration-test".into(),
        })
        .expect("apply defaults path");
    assert_usable_client(client_builder, None).await;
}

/// @covers: apply_defaults
#[tokio::test]
async fn test_apply_defaults_preserves_client_defaults_edge() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(PassThroughLayers);
    let client_builder = layers
        .apply_defaults(ResilientDefaultsRequest {
            builder: builder_with_caller_header(),
            cassette: CassetteConfig::disabled(),
            cassette_name: "integration-test".into(),
        })
        .expect("apply defaults path");
    assert_usable_client(client_builder, Some(b"x-caller: preserved")).await;
}

/// @covers: apply_from_config
#[test]
fn test_apply_from_config_failure_error() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(FailingLayers);
    let loader = ConfigLoaderFactory::create_loader().expect("create config loader");

    let config_error = match layers.apply_from_config(ResilientConfigRequest {
        builder: builder(),
        loader: &loader,
    }) {
        Err(error) => error,
        Ok(_) => panic!("config failure must be returned"),
    };
    assert!(
        matches!(config_error, ResilientError::Config(message) if message == "invalid configuration")
    );
}

/// @covers: apply_defaults
#[test]
fn test_apply_defaults_failure_error() {
    let layers: Arc<dyn ResilientLayerComposer> = Arc::new(FailingLayers);
    let defaults_error = match layers.apply_defaults(ResilientDefaultsRequest {
        builder: builder(),
        cassette: CassetteConfig::disabled(),
        cassette_name: "integration-test".into(),
    }) {
        Err(error) => error,
        Ok(_) => panic!("defaults failure must be returned"),
    };
    assert!(
        matches!(defaults_error, ResilientError::Cassette(message) if message == "invalid cassette")
    );
}

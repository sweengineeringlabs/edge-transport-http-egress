//! End-to-end tests for `DefaultResilientLayers` — proves the composed
//! client (built through the real `retry`/`rate`/`breaker`/`cache`/
//! `cassette` layers, exactly as `transport_svc.rs` used to assemble them
//! internally) actually works against a real server, not just that it
//! compiles.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::convert::Infallible;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;

use edge_transport_http_egress_cassette::CassetteConfig;
use edge_transport_http_egress_resilient::{
    ApplyDefaultsRequest, DefaultResilientLayers, ResilientLayers,
};

async fn spawn_once<F, Fut>(handler: F) -> (u16, tokio::task::JoinHandle<()>)
where
    F: Fn(Request<Incoming>) -> Fut + Send + Clone + 'static,
    Fut: std::future::Future<Output = Response<Full<Bytes>>> + Send,
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let port = listener.local_addr().expect("local_addr").port();
    let jh = tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let io = TokioIo::new(stream);
            let handler = handler.clone();
            tokio::spawn(async move {
                let _ = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |req: Request<Incoming>| {
                            let handler = handler.clone();
                            async move { Ok::<_, Infallible>(handler(req).await) }
                        }),
                    )
                    .await;
            });
        }
    });
    tokio::time::sleep(Duration::from_millis(5)).await;
    (port, jh)
}

/// The composed client (retry/rate/breaker/cache/cassette all wired via
/// `apply_defaults`, cassette disabled) must still round-trip a normal
/// request correctly — proves the chain doesn't break ordinary traffic.
#[tokio::test]
async fn test_apply_defaults_composed_client_round_trips_real_http_call_happy() {
    let (port, _jh) = spawn_once(|req| async move {
        Response::new(Full::new(Bytes::from(req.uri().path().to_string())))
    })
    .await;

    let client = reqwest::Client::builder()
        .build()
        .expect("build reqwest client");
    let builder = reqwest_middleware::ClientBuilder::new(client);
    let builder = DefaultResilientLayers
        .apply_defaults(ApplyDefaultsRequest {
            builder,
            cassette: CassetteConfig::disabled(),
            cassette_name: "e2e-round-trip".to_string(),
        })
        .expect("apply_defaults must succeed")
        .build();

    let resp = builder
        .get(format!("http://127.0.0.1:{port}/widgets/42"))
        .send()
        .await
        .expect("reachable server must succeed");
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.text().await.expect("read body");
    assert_eq!(
        body, "/widgets/42",
        "the composed client must forward the exact request path through every layer unchanged"
    );
}

/// Proves the composed chain's retry layer genuinely retries — not just
/// that the layer object can be constructed. Server fails the first two
/// calls with 503, succeeds on the third; the client must still see 200,
/// proving retry-then-succeed actually happened end-to-end through the
/// real chain (rate/breaker/cache/cassette included, not retry in
/// isolation).
#[tokio::test]
async fn test_apply_defaults_retry_layer_recovers_from_transient_failures_happy() {
    let attempts = Arc::new(AtomicU32::new(0));
    let attempts_for_handler = Arc::clone(&attempts);
    let (port, _jh) = spawn_once(move |_req| {
        let attempts = Arc::clone(&attempts_for_handler);
        async move {
            let n = attempts.fetch_add(1, Ordering::SeqCst);
            if n < 2 {
                Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Full::new(Bytes::from("retry me")))
                    .expect("build 503 response")
            } else {
                Response::new(Full::new(Bytes::from("ok")))
            }
        }
    })
    .await;

    let client = reqwest::Client::builder()
        .build()
        .expect("build reqwest client");
    let builder = reqwest_middleware::ClientBuilder::new(client);
    let builder = DefaultResilientLayers
        .apply_defaults(ApplyDefaultsRequest {
            builder,
            cassette: CassetteConfig::disabled(),
            cassette_name: "e2e-retry".to_string(),
        })
        .expect("apply_defaults must succeed")
        .build();

    let resp = builder
        .get(format!("http://127.0.0.1:{port}/flaky"))
        .send()
        .await
        .expect("request must eventually succeed after retries");
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "the default retry policy must retry a 503 enough times to reach the eventual 200"
    );
    assert_eq!(
        attempts.load(Ordering::SeqCst),
        3,
        "server must have been hit exactly 3 times: 2 failures the retry layer absorbed, plus the succeeding call"
    );
}

/// `apply_from_config` with no `[retry]`/`[rate]`/`[breaker]`/`[cache]`/
/// `[cassette]` sections present must omit every layer — not add them as
/// silent no-ops — and the resulting client must still work normally.
#[tokio::test]
async fn test_apply_from_config_with_no_sections_present_still_round_trips_happy() {
    let (port, _jh) =
        spawn_once(|_req| async move { Response::new(Full::new(Bytes::from("ok"))) }).await;

    let loader = swe_edge_configbuilder::ConfigLoaderFactory::create_loader()
        .expect("create_loader must succeed with no config dirs configured");

    let client = reqwest::Client::builder()
        .build()
        .expect("build reqwest client");
    let builder = reqwest_middleware::ClientBuilder::new(client);
    let builder = DefaultResilientLayers
        .apply_from_config(
            edge_transport_http_egress_resilient::ApplyFromConfigRequest {
                builder,
                loader: &loader,
            },
        )
        .expect("apply_from_config must succeed with zero sections present")
        .build();

    let resp = builder
        .get(format!("http://127.0.0.1:{port}/ping"))
        .send()
        .await
        .expect("request must succeed through an empty (layer-less) chain");
    assert_eq!(resp.status(), StatusCode::OK);
}

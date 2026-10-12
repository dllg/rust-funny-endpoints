use funny_endpoints::{
    client::{HttpClient, Reqwest},
    error::Error,
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method},
};

#[tokio::test]
async fn sends_headers_and_parses_json_despite_content_type() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(header("Accept", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(r#"{"joke":"x"}"#, "text/html"))
        .mount(&server)
        .await;
    let got = Reqwest::default()
        .get(&server.uri(), &[("Accept", "application/json")])
        .await
        .unwrap();
    assert_eq!(got, json!({"joke": "x"}));
}

#[tokio::test]
async fn non_2xx_and_bad_json_are_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;
    assert!(matches!(
        Reqwest::default().get(&server.uri(), &[]).await,
        Err(Error::Status(503))
    ));

    let bad = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>"))
        .mount(&bad)
        .await;
    assert!(matches!(
        Reqwest::default().get(&bad.uri(), &[]).await,
        Err(Error::Json(_))
    ));
}

/// Hits the real internet: `cargo test -- --ignored`. Catches TLS/feature regressions CI cannot.
#[tokio::test]
#[ignore = "needs network"]
async fn live_upstreams() {
    let c = Reqwest::default();
    c.get("https://api.adviceslip.com/advice", &[])
        .await
        .unwrap();
    c.get("https://api.chucknorris.io/jokes/random", &[])
        .await
        .unwrap();
    c.get(
        "https://icanhazdadjoke.com/",
        &[("Accept", "application/json")],
    )
    .await
    .unwrap();
}

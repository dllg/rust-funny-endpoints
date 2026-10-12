use std::collections::HashSet;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use funny_endpoints::{app, client::HttpClient, error::Error};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

/// Canned upstream answers, a failure, or a well-formed reply of the wrong shape.
enum Fake {
    Ok,
    Fail,
    WrongShape,
}

impl HttpClient for Fake {
    async fn get(&self, url: &str, _headers: &[(&str, &str)]) -> Result<Value, Error> {
        match self {
            Self::Fail => return Err(Error::Status(500)),
            Self::WrongShape => return Ok(json!({"unexpected": true})),
            Self::Ok => {}
        }
        Ok(match url {
            u if u.contains("adviceslip") => json!({"slip": {"advice": "be kind"}}),
            u if u.contains("chucknorris") => json!({"value": "roundhouse"}),
            _ => json!({"joke": "talonted"}),
        })
    }
}

async fn call(router: &Router, uri: &str) -> (StatusCode, Value) {
    let resp = router
        .clone()
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn endpoints_return_message() {
    let router = app(Fake::Ok);
    for (uri, want) in [
        ("/v1/advice", "be kind"),
        ("/v1/chucknorris", "roundhouse"),
        ("/v1/dadjoke", "talonted"),
    ] {
        assert_eq!(
            call(&router, uri).await,
            (StatusCode::OK, json!({"message": want}))
        );
    }
}

#[tokio::test]
async fn random_covers_all_sources_with_prefix() {
    let router = app(Fake::Ok);
    let mut seen = HashSet::new();
    for _ in 0..100 {
        let (status, body) = call(&router, "/v1/random").await;
        assert_eq!(status, StatusCode::OK);
        let msg = body["message"].as_str().unwrap().to_owned();
        let prefix = msg.split(": ").next().unwrap().to_owned();
        assert!(
            ["Advice", "Chuck Norris Joke", "Dad Joke"].contains(&prefix.as_str()),
            "{msg}"
        );
        seen.insert(prefix);
    }
    assert_eq!(seen.len(), 3); // miss chance ~7e-18
}

#[tokio::test]
async fn upstream_failure_or_wrong_shape_is_502_with_error_body() {
    for fake in [Fake::Fail, Fake::WrongShape] {
        assert_all_502(&app(fake)).await;
    }
}

async fn assert_all_502(router: &Router) {
    for uri in ["/v1/advice", "/v1/chucknorris", "/v1/dadjoke", "/v1/random"] {
        let (status, body) = call(router, uri).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY, "{uri}");
        assert_eq!(body["errors"][0]["code"], "upstream_error");
    }
}

#[tokio::test]
async fn unknown_route_404_wrong_method_405_spec_served() {
    let router = app(Fake::Ok);
    assert_eq!(call(&router, "/nope").await.0, StatusCode::NOT_FOUND);
    let post = router
        .clone()
        .oneshot(Request::post("/v1/advice").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(post.status(), StatusCode::METHOD_NOT_ALLOWED);
    let (status, spec) = call(&router, "/api-docs/openapi.json").await;
    assert_eq!(status, StatusCode::OK);
    assert!(spec["paths"]["/v1/random"].is_object());
}

//! Funny text from open APIs, served on GET endpoints.

pub mod client;
pub mod error;
pub mod funny;

use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use client::HttpClient;
use error::{Error, ErrorResponse};
use funny::Funny;

#[derive(Serialize, ToSchema)]
pub struct Message {
    message: String,
}

type Shared<C> = State<Arc<Funny<C>>>;
type Reply = Result<Json<Message>, Error>;

fn reply(text: Result<String, Error>) -> Reply {
    text.map(|message| Json(Message { message }))
}

/// Returns a piece of advice from api.adviceslip.com.
#[utoipa::path(get, path = "/v1/advice", responses(
    (status = 200, body = Message), (status = 502, body = ErrorResponse)))]
async fn advice<C: HttpClient>(State(f): Shared<C>) -> Reply {
    reply(f.advice().await)
}

/// Returns a Chuck Norris joke from api.chucknorris.io.
#[utoipa::path(get, path = "/v1/chucknorris", responses(
    (status = 200, body = Message), (status = 502, body = ErrorResponse)))]
async fn chucknorris<C: HttpClient>(State(f): Shared<C>) -> Reply {
    reply(f.chucknorris().await)
}

/// Returns a dad joke from icanhazdadjoke.com.
#[utoipa::path(get, path = "/v1/dadjoke", responses(
    (status = 200, body = Message), (status = 502, body = ErrorResponse)))]
async fn dadjoke<C: HttpClient>(State(f): Shared<C>) -> Reply {
    reply(f.dadjoke().await)
}

/// Returns a random advice, Chuck Norris joke or dad joke, prefixed with its kind.
#[utoipa::path(get, path = "/v1/random", responses(
    (status = 200, body = Message), (status = 502, body = ErrorResponse)))]
async fn random<C: HttpClient>(State(f): Shared<C>) -> Reply {
    reply(f.random_message().await)
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Funny Endpoints API",
        description = "Get funny text by interacting with this API"
    ),
    paths(advice, chucknorris, dadjoke, random)
)]
struct ApiDoc;

/// Builds the router: `/v1/*` endpoints plus Swagger UI at `/swagger-ui`.
pub fn app<C: HttpClient>(client: C) -> Router {
    Router::new()
        .route("/v1/advice", get(advice::<C>))
        .route("/v1/chucknorris", get(chucknorris::<C>))
        .route("/v1/dadjoke", get(dadjoke::<C>))
        .route("/v1/random", get(random::<C>))
        .with_state(Arc::new(Funny::new(client)))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

[![CI](https://github.com/dllg/rust-funny-endpoints/actions/workflows/ci.yml/badge.svg)](https://github.com/dllg/rust-funny-endpoints/actions/workflows/ci.yml)

# rust-funny-endpoints

Simple Rust application that interacts with different open APIs to create text that you can receive on GET endpoints.
Rust sibling of [go-](https://github.com/dllg/go-funny-endpoints), [py-](https://github.com/dllg/py-funny-endpoints) and [cpp-funny-endpoints](https://github.com/dllg/cpp-funny-endpoints).

| Endpoint | Upstream |
|---|---|
| `GET /v1/advice` | [api.adviceslip.com](https://api.adviceslip.com/) |
| `GET /v1/chucknorris` | [api.chucknorris.io](https://api.chucknorris.io) |
| `GET /v1/dadjoke` | [icanhazdadjoke.com](https://icanhazdadjoke.com/) |
| `GET /v1/random` | one of the above, prefixed with its kind |

Success is `200 {"message": "..."}`. If an upstream fails the answer is `502 {"errors":[{"code":"upstream_error","message":"..."}]}`.
Swagger UI is at `/swagger-ui`, the generated OpenAPI spec at `/api-docs/openapi.json`.

Built with [axum](https://github.com/tokio-rs/axum) (server), [reqwest](https://github.com/seanmonstar/reqwest) with rustls (client),
[serde](https://serde.rs), [tracing](https://github.com/tokio-rs/tracing) (logging) and [utoipa](https://github.com/juhaku/utoipa) (OpenAPI).

## Testing

```bash
cargo test                    # offline: handlers use a fake client, the client is tested against wiremock
cargo test -- --ignored       # also hit the real upstreams (checks TLS end to end)
cargo fmt --check && cargo clippy --all-targets -- -D warnings
```

The HTTP client is a trait ([`HttpClient`](src/client.rs)) injected into the service, as in the go/cpp variants. Handlers are tested
with `tower::ServiceExt::oneshot`, the Rust counterpart of go's `httptest`.

## Running

```bash
cargo run --release           # listens on 0.0.0.0:18080
SERVER_PORT=8111 cargo run    # other port
RUST_LOG=debug cargo run      # log level
docker compose up -d --build  # or in docker (alpine image)
```

```bash
$ curl http://localhost:18080/v1/random
{"message":"Dad Joke: What do you call an eagle who can play the piano? Talonted!"}
```

## Docker image

Alpine based (static musl binary, ~27 MB), built for `linux/amd64` and `linux/arm64` by CI; pushed to
`ghcr.io/dllg/rust-funny-endpoints:latest` from `main`. Locally `docker build .` builds for your own architecture.

FROM rust:1-slim-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --no-create-home app
COPY --from=builder /src/target/release/funny-endpoints /usr/local/bin/funny-endpoints
USER app
ENV SERVER_PORT=18080
EXPOSE 18080
HEALTHCHECK CMD curl -fsS http://localhost:${SERVER_PORT}/swagger-ui/ >/dev/null || exit 1
ENTRYPOINT ["funny-endpoints"]

FROM rust:1-alpine AS builder
RUN apk add --no-cache build-base
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM alpine:3
RUN apk add --no-cache ca-certificates curl && adduser -S -H app
COPY --from=builder /src/target/release/funny-endpoints /usr/local/bin/funny-endpoints
USER app
ENV SERVER_PORT=18080
EXPOSE 18080
HEALTHCHECK CMD curl -fsS http://localhost:${SERVER_PORT}/swagger-ui/ >/dev/null || exit 1
ENTRYPOINT ["funny-endpoints"]

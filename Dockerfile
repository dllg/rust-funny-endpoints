FROM rust:1-alpine AS builder
RUN apk add --no-cache build-base
WORKDIR /src
COPY . .
RUN cargo build --release --locked

FROM alpine:3
RUN apk add --no-cache ca-certificates && adduser -S -H app
COPY --from=builder /src/target/release/funny-endpoints /usr/local/bin/funny-endpoints
USER app
ENV SERVER_PORT=18080
EXPOSE 18080
HEALTHCHECK CMD wget -q -O /dev/null http://127.0.0.1:${SERVER_PORT}/swagger-ui/ || exit 1
ENTRYPOINT ["funny-endpoints"]

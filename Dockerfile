# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /endpoint

COPY Cargo.* ./
COPY src ./src/

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest
ARG github_api_key

ENV GITHUB_API_KEY ${github_api_key}

COPY --from=cargo-build /endpoint/target/x86_64-unknown-linux-musl/release/tdd-ccd-endpoint-rs /usr/local/bin/endpoint-rs

CMD ["endpoint-rs"]
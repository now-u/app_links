FROM rust:1-slim-buster AS build
RUN cargo new --bin app
WORKDIR /app
COPY Cargo.toml /app/
COPY Cargo.lock /app/
RUN cargo build --release
COPY src /app/src
COPY migrations /app/migrations
COPY templates /app/templates
RUN touch src/main.rs
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /app/target/release/app_links /app/app_links
CMD "/app/app_links"

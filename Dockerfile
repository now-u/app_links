FROM rust:1.75 as build

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12

COPY —from=build /usr/local/cargo/bin/polylink /usr/local/bin/polylink

CMD [“polylink”]

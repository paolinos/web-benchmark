FROM rust:1.64.0-slim-buster AS build-project
WORKDIR /app

COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

# Production image
FROM rust:1.64.0-slim-buster
WORKDIR /app

COPY --from=build-project /app/target/release/rust-actix .

EXPOSE 3000

CMD ["./rust-actix"]
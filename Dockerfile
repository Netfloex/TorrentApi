FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin api-server

FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/api-server /usr/local/bin/
RUN apt update && apt install -y curl
RUN rm -rf /var/lib/apt/lists/*

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/usr/local/bin/api-server"]
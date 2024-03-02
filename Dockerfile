FROM clux/muslrust:stable AS chef
USER root

RUN cargo install cargo-chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

# RUN cargo fetch --locked
RUN cargo build --release --target x86_64-unknown-linux-musl --bin api-server

FROM alpine AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api-server /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/usr/local/bin/api-server"]
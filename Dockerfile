
ARG RUST_VERSION=1.65

FROM rust:${RUST_VERSION} as builder

WORKDIR /src

COPY . .

RUN cargo fetch --locked
RUN cargo build --release

FROM rust:${RUST_VERSION} as runner

WORKDIR /app

COPY --from=builder /src/target/release/api-server .

CMD ["/app/api-server"]
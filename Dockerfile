# Target architecture
# aarch64-unknown-linux-musl
# x86_64-unknown-linux-musl
ARG TARGET

FROM clux/muslrust:1.77.0-stable AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG TARGET

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --target $TARGET --recipe-path recipe.json
# 
COPY . .

RUN cargo build --release --target $TARGET --bin api-server

FROM scratch AS runtime
ARG TARGET

COPY --from=builder /app/target/$TARGET/release/api-server /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/usr/local/bin/api-server"]
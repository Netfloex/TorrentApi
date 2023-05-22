
ARG RUST_VERSION=1.69

FROM rust:${RUST_VERSION}-alpine as builder

RUN apk add --no-cache gcc musl-dev

WORKDIR /src

COPY . .

RUN cargo fetch --locked
RUN cargo build --release

FROM alpine as runner

WORKDIR /app

COPY --from=builder /src/target/release/api-server .

CMD ["/app/api-server"]
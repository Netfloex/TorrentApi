
ARG RUST_VERSION=1.69

FROM rust:${RUST_VERSION}-alpine as builder

RUN apk add --no-cache gcc musl-dev openssl-dev

WORKDIR /src

COPY . .

RUN cargo fetch --locked
RUN cargo build --release

FROM alpine as runner

WORKDIR /app

COPY --from=builder /src/target/release/api-server .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/app/api-server"]
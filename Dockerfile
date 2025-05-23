FROM clux/muslrust:1.86.0-stable AS chef

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

RUN cargo binstall cargo-chef --no-confirm
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
#
COPY . .

RUN cargo build --release

FROM alpine AS runtime

COPY --from=builder /app/target/*/release/api-server /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/usr/local/bin/api-server"]

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y protobuf-compiler libprotobuf-dev

ARG TARGET="rustc -vV | sed -n 's|host: ||p'"
ENV RUSTFLAGS "-Lnative=/usr/lib -C target-cpu=native"

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin server --target $(eval $TARGET)
RUN mv target/$(eval $TARGET)/release/server target/app

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/app /app

CMD ["./app"]

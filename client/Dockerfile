# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# WORKDIR /app
#
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
#
# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN apt-get update && apt-get -y install cmake
# RUN cargo chef cook --release --recipe-path recipe.json
# COPY . .
# RUN cargo build --release --bin client
#
# FROM gcr.io/distroless/cc-debian12 as runtime
# WORKDIR /app
# COPY --from=builder /app/target/release/client client
# ENTRYPOINT ["./client"]

FROM rust as builder

WORKDIR /usr/src/app
COPY . .
# Will build and cache the binary and dependent crates in release mode
RUN apt-get update && apt-get -y install cmake
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
  --mount=type=cache,target=target \
  cargo build --release && mv ./target/release/client ./client

# Runtime image
FROM debian
RUN apt-get update && apt-get install -y libssl-dev
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*


# Run as "app" user
RUN useradd -ms /bin/bash app
USER app
WORKDIR /app
# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/client /app/client

# Run the app
CMD ./client
EXPOSE 3000

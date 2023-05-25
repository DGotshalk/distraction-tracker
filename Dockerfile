FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
COPY sqlx-data.json ./
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin distraction-tracker

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/distraction-tracker /app/app-bin
COPY --from=builder /app/static/ /app/static
ENTRYPOINT ["/app/app-bin"]
EXPOSE 8080

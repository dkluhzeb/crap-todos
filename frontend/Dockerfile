FROM rust:1.87-bookworm AS chef
RUN cargo install cargo-chef cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Install tailwind
RUN curl -sL https://github.com/nicolo-ribaudo/tailwindcss-cli/releases/latest/download/tailwindcss-linux-x64 -o /usr/local/bin/tailwindcss && chmod +x /usr/local/bin/tailwindcss

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --features ssr --no-default-features
RUN cargo chef cook --release --recipe-path recipe.json --target wasm32-unknown-unknown --features hydrate --no-default-features

COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/crap-todos-frontend /usr/local/bin/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

WORKDIR /app
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["crap-todos-frontend"]

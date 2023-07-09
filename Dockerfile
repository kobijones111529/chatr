FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos

# RUN USER=root cargo new chat-r
# WORKDIR /usr/src/chat-r
# COPY Cargo.toml Cargo.lock ./
# RUN touch src/lib.rs
# RUN mkdir assets
# RUN mkdir style
# RUN touch style/main.scss
# RUN cargo leptos build --release

# COPY src ./src
# COPY style ./style
# COPY assets ./assets
# RUN cargo leptos build --release

COPY . /usr/src/chat-r
WORKDIR /usr/src/chat-r
RUN cargo leptos build --release

FROM debian:stable-slim
COPY --from=builder /usr/src/chat-r/target/server/release/leptos_start .
COPY --from=builder /usr/src/chat-r/target/site ./site
USER 1000
ENV LEPTOS_OUTPUT_NAME="leptos_start"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
CMD [ "./leptos_start" ]

FROM rust:1.78.0-slim-bullseye

WORKDIR /usr/src/htmx-in-rust
COPY . .

RUN cargo install --path .

CMD ["htmx-in-rust"]
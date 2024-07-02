FROM rust:1.78.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/htmx-in-rust
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

EXPOSE 8080

COPY --from=build /usr/local/cargo/bin/htmx-in-rust /usr/local/bin/htmx-in-rust

CMD ["htmx-in-rust"]
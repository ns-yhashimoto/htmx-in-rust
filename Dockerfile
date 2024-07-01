FROM rust:1.78.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PORT 80

WORKDIR /usr/src/htmx-in-rust
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/htmx-in-rust /usr/local/bin/htmx-in-rust

ENV PORT 80
EXPOSE 80

CMD ["htmx-in-rust"]
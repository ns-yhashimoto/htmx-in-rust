FROM rust:1.60.0 AS build-stage
RUN USER=root cargo new --bin basics
COPY ./ /basics
RUN cargo build --release

FROM debian:bullseye-slim AS production
COPY --from=build-stage /basics/target/release/basics .
COPY --from=build-stage /basics/static/ /static
CMD ["./basics"]
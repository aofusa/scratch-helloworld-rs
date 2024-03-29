FROM rust:1-slim-bullseye AS builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/scratch-helloworld /hello-world
CMD ["/hello-world"]


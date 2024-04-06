FROM rust:1.67 as builder
WORKDIR /backend
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend
ENTRYPOINT ["backend"]

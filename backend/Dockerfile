FROM rust:1-buster as builder
WORKDIR /backend
COPY . .
RUN cargo install sqlx-cli
RUN sqlx migrate run
RUN cargo sqlx prepare
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend
EXPOSE 3001
ENTRYPOINT ["backend"]

FROM rust:1-buster as builder
WORKDIR /backend
ENV SQLX_OFFLINE true

COPY . .

RUN cargo build --release


FROM debian:buster-slim
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend
EXPOSE 3001
ENTRYPOINT ["backend"]

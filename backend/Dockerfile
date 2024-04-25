FROM rust:1.77.1 as builder
WORKDIR /backend

COPY . .


RUN cargo build --release


FROM rust:1.77.1
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY --from=builder /backend/target/release/backend /usr/local/bin/backend
EXPOSE 3001
ENTRYPOINT ["backend"]

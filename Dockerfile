FROM rust:1.74.1

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . /app
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]
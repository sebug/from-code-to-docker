FROM rust:1.61.0-alpine AS builder

RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev

RUN rustup target add aarch64-unknown-linux-musl
RUN rustup toolchain install stable-aarch64-unknown-linux-musl

RUN mkdir -p /usr/src/robernetes

WORKDIR /usr/src/robernetes

COPY Cargo.toml Cargo.lock ./

COPY ./hello.html ./

COPY ./src ./src

RUN cargo build --target aarch64-unknown-linux-musl --release

RUN ls /usr/src/robernetes/target/aarch64-unknown-linux-musl/release

# Bundle Stage
FROM scratch
COPY --from=builder /usr/src/robernetes/hello.html .
COPY --from=builder /usr/src/robernetes/target/aarch64-unknown-linux-musl/release/app ./robernetes

EXPOSE 8090

CMD ["./robernetes"]
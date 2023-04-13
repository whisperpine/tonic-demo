# tonic_demo_client

FROM rust:alpine AS builder
RUN apk add libc-dev
WORKDIR /project/
COPY .cargo/config.toml ./.cargo/
COPY Cargo.toml .
RUN cargo fetch
COPY . .
RUN cargo build --offline --release

FROM scratch
COPY --from=builder /project/target/release/tonic_demo_client /app/
EXPOSE 3000
CMD [ "/app/tonic_demo_client" ]
#syntax=docker/dockerfile:1.2
FROM rust:latest as builder

WORKDIR /workspace
COPY . .

RUN --mount=type=cache,id=cargo-target,target=/workspace/target \
    --mount=type=cache,id=cargo-registry-cache,target=/usr/local/cargo/registry \
    cargo build --release && cp target/release/francis bin

FROM gcr.io/distroless/cc as service

WORKDIR /app
COPY --from=builder /workspace/bin /app/bin

CMD ["/app/bin"]


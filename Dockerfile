# Multi-stage Docker for neuromod (library + examples/tests)
# Builder — Rust 1.98.0 (bookworm) keeps glibc in sync with the debian:bookworm-slim runtime.
# Toolchain pin (Cargo.toml/rust-toolchain.toml/CI) is 1.98.1; this stays at 1.98.0 until
# the upstream `rust:1.98.1-slim-bookworm` image is published.
FROM rust:1.98.0-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --examples && \
    mkdir -p /out && \
    find target/release/examples/ -maxdepth 1 -type f -executable -exec cp {} /out/ \;

# Runtime example (minimal)
FROM debian:bookworm-slim
RUN useradd --system --create-home --shell /usr/sbin/nologin neuromod
WORKDIR /app
COPY --from=builder /out/ /usr/local/bin/
USER neuromod
# For library usage, typically users depend on the crate, not the image.
# This image is useful for CI reproducibility and example runs.
CMD ["ls", "/usr/local/bin"]

# Multi-stage Docker for neuromod (library + examples/tests)
# Builder — bookworm base image keeps glibc in sync with the debian:bookworm-slim runtime.
# The base image itself is rust:1.98.0-slim-bookworm (latest published tag; 1.98.1 has none
# yet), but `COPY . .` brings in rust-toolchain.toml, so rustup fetches and uses the pinned
# 1.98.1 toolchain for the actual `cargo build` below — the base image only supplies rustup
# and the OS layer, not the toolchain version that compiles the crate.
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

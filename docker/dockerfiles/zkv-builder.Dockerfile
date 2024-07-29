FROM rust:1-bookworm

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    protobuf-compiler \
    cmake \
    clang && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src && \
    # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete;

WORKDIR /usr/src/node

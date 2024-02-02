FROM rust:1-buster

COPY /docker/resources/protoc-22.0-linux-x86_64.zip /
RUN 

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    clang && \
    unzip protoc-22.0-linux-x86_64.zip -d /usr/local/ && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src && \
    # apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete;

# RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/node

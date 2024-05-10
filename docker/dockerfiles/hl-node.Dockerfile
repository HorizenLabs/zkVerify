FROM rust:1-buster as builder

RUN apt-get update -qq \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  protobuf-compiler \
  clang \
  && rustup target add wasm32-unknown-unknown \
  && rustup component add rust-src \
  && apt-get -y clean \
  && apt-get -y autoclean \
  && apt-get -y autoremove \
  && rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*.deb

ARG PROFILE="release"
ARG PROFILE="release"
ARG FEATURES=""

WORKDIR /usr/src/node
COPY . .
RUN cargo build --profile ${PROFILE} --features "${FEATURES}"

FROM ubuntu:22.04 as node

SHELL ["/bin/bash", "-c"]

# That can be a single one or a comma separated list
ARG BINARY=nh-node
ARG DESCRIPTION="New Horizen Core"
ARG AUTHORS="mainchain-team@horizenlabs.io"
ARG VENDOR="Horizen Labs"
ARG PROFILE="release"
ARG FEATURES=""

ENV BINARY="${BINARY}" \
  RUN_USER=hl

LABEL io.hl.image.authors="${AUTHORS}" \
  io.hl.image.vendor="${VENDOR}" \
  io.hl.image.description="${DESCRIPTION}" \
  io.hl.image.profile="${PROFILE}" \
  io.hl.image.features="${FEATURES}"

USER root
WORKDIR /app

COPY docker/scripts/entrypoint.sh .
COPY --from=builder "/usr/src/node/target/${PROFILE}/nh-node" "/usr/local/bin/"
COPY --from=builder "/usr/src/node/target/${PROFILE}/wbuild/nh-runtime/nh_runtime.compact.compressed.wasm" "./nh_runtime.compact.compressed.wasm"
RUN chmod -R a+rx "/usr/local/bin"

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  curl \
  && useradd -m -u 1000 -U -s /bin/sh -d /${RUN_USER} ${RUN_USER} \
  && mkdir -p /data /${RUN_USER}/.local/share \
  && chown -R ${RUN_USER}:${RUN_USER} /data /${RUN_USER} \
  && ln -s /data /${RUN_USER}/.local/share \
  && apt-get -y clean \
  && apt-get -y autoclean \
  && apt-get -y autoremove \
  && rm -rf /var/{lib/apt/lists/*,cache/apt/archives/*.deb} /tmp/*

USER ${RUN_USER}

# ENTRYPOINT
ENTRYPOINT ["/app/entrypoint.sh"]
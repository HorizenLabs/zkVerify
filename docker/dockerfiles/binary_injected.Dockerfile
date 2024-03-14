FROM ubuntu:22.04

SHELL ["/bin/bash", "-c"]

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

# That can be a single one or a comma separated list
ARG BINARY=nh-node

ARG BIN_FOLDER=.
ARG DOC_URL=https://github.com/HorizenLabs/NH-core
ARG DESCRIPTION="NH-core"
ARG AUTHORS="devops@horizenlabs.io"
ARG VENDOR="Horizen Labs"

LABEL io.hl.image.authors=${AUTHORS} \
	io.hl.image.vendor="${VENDOR}" \
	io.hl.image.revision="${VCS_REF}" \
	io.hl.image.title="${IMAGE_NAME}" \
	io.hl.image.created="${BUILD_DATE}" \
	io.hl.image.documentation="${DOC_URL}" \
	io.hl.image.description="${DESCRIPTION}" \
	io.hl.image.source="https://github.com/HorizenLabs/NH-core/blob/${VCS_REF}/docker/dockerfiles/binary_injected.Dockerfile"

USER root
WORKDIR /app

COPY entrypoint.sh .
COPY "bin/*" "/usr/local/bin/"
RUN chmod -R a+rx "/usr/local/bin"

ENV RUN_USER hl

RUN apt-get update && 	\
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
	libssl3 ca-certificates gnupg curl && \
	apt-get autoremove -y && \
	apt-get clean && \
	useradd -m -u 1000 -U -s /bin/sh -d /${RUN_USER} ${RUN_USER} && \
	rm -rf /var/lib/apt/lists/* ; 	mkdir -p /data /${RUN_USER}/.local/share && \
	chown -R ${RUN_USER}:${RUN_USER} /data /${RUN_USER} && \
	ln -s /data /${RUN_USER}/.local/share

USER ${RUN_USER}
ENV BINARY=${BINARY}

# ENTRYPOINT
ENTRYPOINT ["/app/entrypoint.sh"]

# We call the help by default
# CMD ["--help"]

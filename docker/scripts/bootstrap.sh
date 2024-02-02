#!/bin/sh
# shellcheck disable=SC2086

PROJECT_ROOT=${PROJECT_ROOT:-$(git rev-parse --show-toplevel)}
SCRIPTS=${PROJECT_ROOT}/docker/scripts
DOCKERS=${PROJECT_ROOT}/docker/dockerfiles
CARGO=${SCRIPTS}/my_cargo

# Build rbuilder
echo "----------------------------------------------------------"
echo "Building rbuilder"
docker build -f ${DOCKERS}/hl-builder.Dockerfile -t rbuilder ${PROJECT_ROOT}

# Compile node
echo "----------------------------------------------------------"
echo "Compile node"
${CARGO} build --release 

# Create node image
echo "----------------------------------------------------------"
echo "Building node image"
${SCRIPTS}/build-chain-image-injected.sh

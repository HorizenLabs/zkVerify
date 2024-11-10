#!/bin/sh
set -e

PROJECT_ROOT=${PROJECT_ROOT:-$(git rev-parse --show-toplevel)}
SCRIPTS=${PROJECT_ROOT}/docker/scripts
DOCKERS=${PROJECT_ROOT}/docker/dockerfiles
CARGO=${SCRIPTS}/my_cargo
BUILD_PROFILE="${BUILD_PROFILE:---release}"

SOLO_ONLY=false
RELAY_ONLY=false

# Features
FAST_RUNTIME="${FAST_RUNTIME:-true}"                    # for dev, limit an epoch to 1min. Useful for testing with parachains

# Build rbuilder
echo "----------------------------------------------------------"
echo "Building rbuilder"
docker build -f ${DOCKERS}/zkv-builder.Dockerfile -t rbuilder ${PROJECT_ROOT}

echo "SOLO_ONLY=${SOLO_ONLY}"
echo "RELAY_ONLY=${RELAY_ONLY}"

RELAY_FEATURES=""

if [ "$FAST_RUNTIME" = "true" ]; then
  RELAY_FEATURES="fast-runtime"
fi

if [ "$RELAY_FEATURES" ]; then
  RELAY_FEATURES="--features ${RELAY_FEATURES}"
fi

# Determine what to compile/build
while [[ $# -gt 0 ]]; do
    case "$1" in
        --solo-only)
            SOLO_ONLY=true
            shift
            ;;
        --relay-only)
            RELAY_ONLY=true
            shift
            ;;
    esac
done

echo "SOLO_ONLY=${SOLO_ONLY}"
echo "RELAY_ONLY=${RELAY_ONLY}"

# Compile nodes
if [ "${RELAY_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Compile solo node"
  ${CARGO} build -p mainchain "${BUILD_PROFILE}"
fi

if [ "${SOLO_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Compile relay node"
  ${CARGO} build -p zkv-relay "${BUILD_PROFILE}" "${RELAY_FEATURES}"
fi

if [ "${RELAY_ONLY}" != "true" ] && [ "${SOLO_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Compile test parachain node"
  ${CARGO} build -p paratest-node "${BUILD_PROFILE}"
fi

# Create docker images
if [ "${RELAY_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Building solo node image"
  "${SCRIPTS}/build-chain-image-injected.sh"
fi

if [ "${SOLO_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Building relay node image"
  "${SCRIPTS}/build-zkv-relay-image-injected.sh"
fi

if [ "${RELAY_ONLY}" != "true" ] && [ "${SOLO_ONLY}" != "true" ]; then
  echo "----------------------------------------------------------"
  echo "Building parachain node image"
  "${SCRIPTS}/build-paratest-image-injected.sh"
fi

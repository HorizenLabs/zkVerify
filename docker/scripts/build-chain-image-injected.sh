#!/bin/sh

PROJECT_ROOT=${PROJECT_ROOT:-$(git rev-parse --show-toplevel)}

BINARY="nh-node" \
ARTIFACTS_FOLDER=target/release \
"${PROJECT_ROOT}"/docker/scripts/build-injected.sh
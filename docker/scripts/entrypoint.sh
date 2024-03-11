#!/usr/bin/env bash

# Sanity check
if [ -z "$BINARY" ]
then
    echo "BINARY ENV not defined, this should never be the case. Aborting..."
    exit 1
fi

# If the user built the image with multiple binaries,
# we consider the first one to be the canonical one
# To start with another binary, the user can either:
#  - use the --entrypoint option
#  - pass the ENV BINARY with a single binary
IFS=',' read -r -a BINARIES <<< "$BINARY"
NODE=${BINARIES[0]}

NODE_NAME=${NODE_NAME:-"MyNode"}
BASE_PATH=${BASE_PATH:-"/data/node"}
SPEC_PATH=${SPEC_PATH:-"/data/chain_spec.json"}
SEED_PHRASE_PATH=${SEED_PHRASE_PATH:-"/data/seed_phrase"}
NODE_KEY_PATH=${NODE_KEY_PATH:-"/data/node_key.dat"}

ARGS=

SPEC_RAW_PATH="/data/chain_spec_raw.json"

echo "NODE_NAME=${NODE_NAME}"
echo "NODE=${NODE}"
echo "BASE_PATH=${BASE_PATH}"
echo "SPEC_PATH=${SPEC_PATH}"
echo "SEED_PHRASE_PATH=${SEED_PHRASE_PATH}"
echo "SPEC_RAW_PATH=${SPEC_RAW_PATH}"
echo "NODE_KEY_PATH=${NODE_KEY_PATH}"
ls -la "${SPEC_PATH}"

if [ ! -f "${SPEC_PATH}" ]; 
then 
	echo "BUILDING SPECS"
	${NODE} build-spec --disable-default-bootnode --chain local > ${SPEC_PATH}
fi

echo "BUILDING SPECS RAW"

${NODE} build-spec --chain="${SPEC_PATH}" \
    --raw --disable-default-bootnode > ${SPEC_RAW_PATH}

if [ -f ${SEED_PHRASE_PATH} ] ; then
	echo "INJECT KEYS"
	${NODE} key insert --base-path "${BASE_PATH}" \
		--chain "${SPEC_RAW_PATH}" \
    		--scheme Sr25519 \
		--suri "${SEED_PHRASE_PATH}" \
		--key-type aura

	${NODE} key insert --base-path "${BASE_PATH}" \
		--chain "${SPEC_RAW_PATH}" \
    		--scheme Ed25519 \
		--suri "${SEED_PHRASE_PATH}" \
		--key-type gran
fi

if [ -f ${NODE_KEY_PATH} ] ; then
	echo "USE node-key-file"
	cp ${NODE_KEY_PATH} /tmp/node-key.dat
	ARGS="${ARGS} --node-key-file /tmp/node-key.dat"
fi	

exec ${NODE} \
    --base-path "${BASE_PATH}" \
    --chain "${SPEC_RAW_PATH}" \
    --name "${NODE_NAME}" \
    ${ARGS} \
    "$@"

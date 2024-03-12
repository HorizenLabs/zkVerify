#!/usr/bin/env bash

# Sanity check
if [ -z "$BINARY" ]; then
    echo "BINARY ENV not defined, this should never be the case. Aborting..."
    exit 1
fi

# If the user built the image with multiple binaries,
# we consider the first one to be the canonical one
# To start with another binary, the user can either:
#  - use the --entrypoint option
#  - pass the ENV BINARY with a single binary
IFS=',' read -r -a BINARIES <<< "$BINARY"
NH_NODE=${BINARIES[0]}
echo "NH_NODE=${NH_NODE}"

NH_SPEC_PATH=${NH_SPEC_PATH:-"/data/chain_spec.json"}
NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH:-"/data/secret_phrase.dat"}
echo "NH_SPEC_PATH=${NH_SPEC_PATH}"
echo "NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH}"

# Node configurations (env->arg)
NH_CONF_NAME=${NH_CONF_NAME:-"MyNode"}
NH_CONF_BASE_PATH=${NH_CONF_BASE_PATH:-"/data/node"}
NH_CONF_CHAIN=${NH_CONF_CHAIN:-"${NH_SPEC_PATH%/*}/chain_spec_raw.json"}
NH_CONF_VALIDATOR=${NH_CONF_VALIDATOR:-""}
NH_CONF_NODE_KEY_FILE=${NH_CONF_NODE_KEY_FILE:-""}
NH_CONF_BOOTNODES=${NH_CONF_BOOTNODES:-""}
NH_CONF_RPC_CORS=${NH_CONF_RPC_CORS:-""}
NH_CONF_RPC_EXTERNAL=${NH_CONF_RPC_EXTERNAL:-""}
echo "NH_CONF_NAME=${NH_CONF_NAME}"
echo "NH_CONF_BASE_PATH=${NH_CONF_BASE_PATH}"
echo "NH_CONF_CHAIN=${NH_CONF_CHAIN}"
echo "NH_CONF_VALIDATOR=${NH_CONF_VALIDATOR}"
echo "NH_CONF_NODE_KEY_FILE=${NH_CONF_NODE_KEY_FILE}"
echo "NH_CONF_BOOTNODES=${NH_CONF_BOOTNODES}"
echo "NH_CONF_RPC_CORS=${NH_CONF_RPC_CORS}"
echo "NH_CONF_RPC_EXTERNAL=${NH_CONF_RPC_EXTERNAL}"

if [ ! -f "${NH_SPEC_PATH}" ]; then
	echo "Spec file missing. Aborting..."
    exit 1
fi

${NH_NODE} build-spec --chain="${NH_SPEC_PATH}" --raw --disable-default-bootnode > ${NH_CONF_CHAIN}

if [ -f ${NH_SECRET_PHRASE_PATH} ]; then
	echo "Injecting key (Aura)"
	${NH_NODE} key insert --base-path "${NH_CONF_BASE_PATH}" \
		--chain "${NH_CONF_CHAIN}" \
    	--scheme Sr25519 \
		--suri "${NH_SECRET_PHRASE_PATH}" \
		--key-type aura
	echo "Injecting key (Grandpa)"
	${NH_NODE} key insert --base-path "${NH_CONF_BASE_PATH}" \
		--chain "${NH_CONF_CHAIN}" \
    	--scheme Ed25519 \
		--suri "${NH_SECRET_PHRASE_PATH}" \
		--key-type gran
fi

ARGS=
# This is a workaround due to the node needing write permission on the node-key file
if [ -f ${NH_CONF_NODE_KEY_FILE} ]; then
	echo "Copying node key file"
	cp ${NH_CONF_NODE_KEY_FILE} /tmp/node_key.dat
	ARGS+=" --node-key-file /tmp/node_key.dat"
fi
# Set node-specific configurations
if [[ -n $NH_CONF_VALIDATOR && $NH_CONF_VALIDATOR != "XXXXXXXXXX" ]]; then
	ARGS+=" --validator"
fi
if [[ -n $NH_CONF_BOOTNODES && $NH_CONF_VALIDATOR != "XXXXXXXXXX" ]]; then
	ARGS+=" --bootnodes ${NH_CONF_BOOTNODES}"
fi
if [[ -n $NH_CONF_RPC_CORS && $NH_CONF_VALIDATOR != "XXXXXXXXXX" ]]; then
	ARGS+=" --rpc-cors ${NH_CONF_RPC_CORS}"
fi
if [[ -n $NH_CONF_RPC_EXTERNAL && $NH_CONF_VALIDATOR != "XXXXXXXXXX" ]]; then
	ARGS+=" --rpc-external"
fi
echo "ARGS=${ARGS}"

# Check files
echo "Checking files"
find /data -type f
echo ""

exec ${NH_NODE} \
    --base-path "${NH_CONF_BASE_PATH}" \
    --chain "${NH_CONF_CHAIN}" \
    --name "${NH_CONF_NAME}" \
    ${ARGS}

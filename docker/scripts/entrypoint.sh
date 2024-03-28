#!/usr/bin/env bash
set -eEuo pipefail

# Sanity check
if [ -z "${BINARY}" ]; then
    echo "BINARY ENV not defined, this should never be the case. Aborting..."
    exit 1
fi

# If the user built the image with multiple binaries,
# we consider the first one to be the canonical one
# To start with another binary, the user can either:
#  - use the --entrypoint option
#  - pass the ENV BINARY with a single binary
IFS=',' read -r -a BINARIES <<< "$BINARY"
NH_NODE="${BINARIES[0]}"
echo "NH_NODE=${NH_NODE}"

NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH:-"/data/secret_phrase.dat"}
echo "NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH}"

# Node configurations (env->arg)
NH_CONF_NAME=${NH_CONF_NAME:-"MyNode"}
NH_CONF_BASE_PATH=${NH_CONF_BASE_PATH:-"/data/node"}
NH_CONF_CHAIN=${NH_CONF_CHAIN:-"test"}
NH_CONF_VALIDATOR=${NH_CONF_VALIDATOR:-}
NH_CONF_NODE_KEY_FILE=${NH_CONF_NODE_KEY_FILE:-}
NH_CONF_BOOTNODES=${NH_CONF_BOOTNODES:-}
NH_CONF_RPC_CORS=${NH_CONF_RPC_CORS:-}
NH_CONF_RPC_EXTERNAL=${NH_CONF_RPC_EXTERNAL:-}
NH_CONF_RPC_METHODS=${NH_CONF_RPC_METHODS:-}
NH_CONF_PRUNING=${NH_CONF_PRUNING:-}
NH_PROMETHEUS_EXTERNAL=${NH_PROMETHEUS_EXTERNAL:-}

for var_name in NH_CONF_NAME NH_CONF_BASE_PATH NH_CONF_CHAIN NH_CONF_VALIDATOR NH_CONF_NODE_KEY_FILE NH_CONF_BOOTNODES NH_CONF_RPC_CORS NH_CONF_RPC_EXTERNAL NH_CONF_RPC_METHODS NH_CONF_PRUNING NH_PROMETHEUS_EXTERNAL; do
  # Get the value of the variable
  var_value="${!var_name}"

  # Check if the variable is defined
  if [ -n "${var_value}" ]; then
    echo "${var_name}=${var_value}"
  else
    echo "${var_name} is empty"
  fi
done

if [ -f "${NH_SECRET_PHRASE_PATH}" ]; then
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
  echo "Injecting key (Imonline)"
  ${NH_NODE} key insert --base-path "${NH_CONF_BASE_PATH}" \
    --chain "${NH_CONF_CHAIN}" \
    --scheme Sr25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type imon
fi

ARGS=
# This is a workaround due to the node needing write permission on the node-key file
if [ -f "${NH_CONF_NODE_KEY_FILE}" ]; then
	echo "Copying node key file"
	cp "${NH_CONF_NODE_KEY_FILE}" /tmp/node_key.dat
	ARGS+=" --node-key-file /tmp/node_key.dat"
fi

# Set node-specific configurations
if [[ -n "${NH_CONF_VALIDATOR}" && "${NH_CONF_VALIDATOR}" == "true" ]]; then
	ARGS+=" --validator"
else
  if [ -n "${NH_CONF_RPC_CORS}" ]; then
    ARGS+=" --rpc-cors ${NH_CONF_RPC_CORS}"
  fi
  if [[ -n "${NH_CONF_RPC_EXTERNAL}" && "${NH_CONF_RPC_EXTERNAL}" == "true" ]]; then
   	ARGS+=" --rpc-external"
  fi
  if [ -n "${NH_CONF_RPC_METHODS}" ]; then
   	ARGS+=" --rpc-methods ${NH_CONF_RPC_METHODS}"
  fi
  if [ -n "${NH_CONF_PRUNING}" ]; then
   	ARGS+=" --pruning ${NH_CONF_PRUNING}"
  fi
fi
if [ -n "${NH_CONF_BOOTNODES}" ]; then
	ARGS+=" --bootnodes ${NH_CONF_BOOTNODES}"
fi
if [ -n "${NH_PROMETHEUS_EXTERNAL}" ]; then
	ARGS+=" --prometheus-external ${NH_PROMETHEUS_EXTERNAL}"
fi
# append other extra args
ARGS+=" "
ARGS+="$@"

echo "ARGS=${ARGS}"

echo ""

exec "${NH_NODE}" \
    --base-path "${NH_CONF_BASE_PATH}" \
    --chain "${NH_CONF_CHAIN}" \
    --name "${NH_CONF_NAME}" \
    ${ARGS}
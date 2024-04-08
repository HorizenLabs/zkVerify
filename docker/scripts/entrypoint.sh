#!/usr/bin/env bash
set -eEuo pipefail

get_arg_name_from_env_name() {
    local env_name="$1"
    local prefix="$2"
    arg_name="${env_name:${#prefix}}"
    arg_name="${arg_name//_/-}"
    arg_name="${arg_name,,}"
    arg_name=--"${arg_name}"
    echo "${arg_name}"
}

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
prefix="NH_CONF_"
conf_args=""
echo "Node configuration:"
while IFS='=' read -r -d '' var_name var_value; do
  if [[ "$var_name" == NH_CONF* ]]; then
    arg_name=$(get_arg_name_from_env_name "${var_name}" "${prefix}")
    arg_value=""
    if [ "$var_value" != "true" ]; then
      arg_value=" ${var_value}"
    fi
    arg="${arg_name}${arg_value}"
    conf_args+="${arg} "
    echo "  ${var_name}=${var_value} -> $arg"
  fi
done < <(env -0)

# Keys handling
if [ -f "${NH_SECRET_PHRASE_PATH}" ]; then
  path=""
  if [ -z "${NH_CONF_BASE_PATH+}" ]; then
    path=$(get_arg_name_from_env_name NH_CONF_BASE_PATH "${prefix}") $NH_CONF_BASE_PATH
  fi
  chain=""
  if [ -z "${NH_CONF_CHAIN+}" ]; then
    path=$(get_arg_name_from_env_name NH_CONF_CHAIN "${prefix}") $NH_CONF_CHAIN
  fi
  echo "Injecting key (Aura)"
  ${NH_NODE} key insert "${path}" \
    "${chain}" \
    --scheme Sr25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type aura
  echo "Injecting key (Grandpa)"
  ${NH_NODE} key insert "${path}" \
    "${chain}" \
    --scheme Ed25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type gran
  echo "Injecting key (Imonline)"
  ${NH_NODE} key insert  "${path}" \
    "${chain}" \
    --scheme Sr25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type imon
fi

echo "Launching ${NH_NODE} with args ${conf_args}"
exec "${NH_NODE}" ${conf_args}
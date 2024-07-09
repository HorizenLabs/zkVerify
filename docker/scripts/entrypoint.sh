#!/usr/bin/env bash

# This script performs the following tasks:
# 
# - translation of environment variables to command line arguments
# - preparation before the node start (example keys injection)
# - launch of the actual node
# 
# Environment variables should generally be in the form `ZKV_*`
# Environment variables in the form `ZKV_CONF_*` are translated to command line arguments based on these rules:
#
# 1. `ZKV_CONF_` prefix is removed
# 2. if both a trailing underscore (`_`) and number are present, they are removed
# 3. if underscores (`_`) are present, they are replaced with dashes (`-`)
# 4. letters are replaced with lower case
# 5. prefix `--` is added
# 
# Examples:
# 
# - `ZKV_CONF_BASE_PATH` -> `--base-path`
# - `ZKV_CONF_BOOTNODES` -> `--bootnodes`
# - `ZKV_CONF_BOOTNODES_2` -> `--bootnodes`
#
# Values of environment variables are used unmodified as values of command line arguments with the exception
# of `true` being dropped (as a flag, example `ZKV_CONF_VALIDATOR`/`--validator`)

set -eEuo pipefail

get_arg_name_from_env_name() {
    local env_name="$1"
    local prefix="$2"
    arg_name="${env_name:${#prefix}}"
    arg_name=$(echo "${arg_name}" | sed -r 's/^(.+)_[0-9]*$/\1/')
    arg_name="${arg_name//_/-}"
    arg_name="${arg_name,,}"
    arg_name=--"${arg_name}"
    echo "${arg_name}"
}

get_arg_value_from_env_value() {
    local env_value="$1"
    arg_value="${env_value}"
    if [ "$arg_value" == "true" ]; then
      arg_value=""
    fi
    echo "${arg_value}"
}

# Sanity check
if [ -z "${BINARY:-}" ]; then
    echo "BINARY ENV not defined, this should never be the case. Aborting..."
    exit 1
fi

# If the user built the image with multiple binaries,
# we consider the first one to be the canonical one
# To start with another binary, the user can either:
#  - use the --entrypoint option
#  - pass the ENV BINARY with a single binary
IFS=',' read -r -a BINARIES <<< "$BINARY"
ZKV_NODE="${BINARIES[0]}"
echo "ZKV_NODE=${ZKV_NODE}"

ZKV_SECRET_PHRASE_PATH=${ZKV_SECRET_PHRASE_PATH:-"/data/config/secret_phrase.dat"}
echo "ZKV_SECRET_PHRASE_PATH=${ZKV_SECRET_PHRASE_PATH}"
ZKV_NODE_KEY_FILE=${ZKV_NODE_KEY_FILE:-"/data/config/node_key.dat"}
echo "ZKV_NODE_KEY_FILE=${ZKV_NODE_KEY_FILE}"

ZKV_CONF_BASE_PATH=${ZKV_CONF_BASE_PATH:-}
ZKV_CONF_CHAIN=${ZKV_CONF_CHAIN:-}

# Node configurations (env->arg)
prefix="ZKV_CONF_"
conf_args=()
echo "Node configuration:"
while IFS='=' read -r -d '' var_name var_value; do
  if [[ "$var_name" == ${prefix}* ]]; then
    # rules above
    arg_name=$(get_arg_name_from_env_name "${var_name}" "${prefix}")
    conf_args+=("${arg_name}")
    # rules above
    arg_value=$(get_arg_value_from_env_value "${var_value}")
    if [ -n "${arg_value}" ]; then
      conf_args+=("${arg_value}")
    fi
    echo "  ${var_name}=${var_value} -> ${arg_name} ${arg_value}"
  fi
done < <(env -0)

# Realychain's collator configurations (env->arg)
prefix="RC_CONF_"
echo "Relaycain's collator configuration:"
relaychain_appended_any=""
while IFS='=' read -r -d '' var_name var_value; do
  if [[ "$var_name" == ${prefix}* ]]; then
    if [[ -z ${relaychain_appended_any} ]]; then
      relaychain_appended_any="true"
      # Add separator
      conf_args+=("--")  
    fi
    # rules above
    arg_name=$(get_arg_name_from_env_name "${var_name}" "${prefix}")
    conf_args+=("${arg_name}")
    # rules above
    arg_value=$(get_arg_value_from_env_value "${var_value}")
    if [ -n "${arg_value}" ]; then
      conf_args+=("${arg_value}")
    fi
    echo "  ${var_name}=${var_value} -> ${arg_name} ${arg_value}"
  fi
done < <(env -0)

if [ -n "${ZKV_CONF_BASE_PATH}" ]; then
  BASE_CHAINS="${ZKV_CONF_BASE_PATH}/chains"

  for chain in local testnet ; do
    source_chain_dir="${BASE_CHAINS}/nh_${chain}";
    dest_chain_dir="${BASE_CHAINS}/zkv_${chain}";
    [ -d "$source_chain_dir" ] && [ ! -e "$dest_chain_dir" ] && \
      echo "Move ${source_chain_dir} to ${dest_chain_dir}" && \
      mv "${source_chain_dir}" "${dest_chain_dir}"
  done
fi

# Keys handling
if [ -f "${ZKV_SECRET_PHRASE_PATH}" ]; then
  injection_args=()
  if [ -n "${ZKV_CONF_BASE_PATH}" ]; then
    injection_args+=("$(get_arg_name_from_env_name ZKV_CONF_BASE_PATH ${prefix})")
    injection_args+=("$(get_arg_value_from_env_value "${ZKV_CONF_BASE_PATH}")")
  fi
  if [ -n "${ZKV_CONF_CHAIN}" ]; then
    injection_args+=("$(get_arg_name_from_env_name ZKV_CONF_CHAIN ${prefix})")
    injection_args+=("$(get_arg_value_from_env_value "${ZKV_CONF_CHAIN}")")
  fi
  echo "Injecting keys with ${injection_args[*]}"
  echo "Injecting key (Babe)"
  ${ZKV_NODE} key insert "${injection_args[@]}" \
    --scheme Sr25519 \
    --suri "${ZKV_SECRET_PHRASE_PATH}" \
    --key-type babe
  echo "Injecting key (Grandpa)"
  ${ZKV_NODE} key insert "${injection_args[@]}" \
    --scheme Ed25519 \
    --suri "${ZKV_SECRET_PHRASE_PATH}" \
    --key-type gran
  echo "Injecting key (Imonline)"
  ${ZKV_NODE} key insert "${injection_args[@]}" \
    --scheme Sr25519 \
    --suri "${ZKV_SECRET_PHRASE_PATH}" \
    --key-type imon
fi

# Node-key handling
if [[ (-n "${ZKV_CONF_BASE_PATH}") && (-n "${ZKV_CONF_CHAIN}") && (-f "${ZKV_NODE_KEY_FILE}") ]]; then
  base_path=("$(get_arg_value_from_env_value "${ZKV_CONF_BASE_PATH}")")
  chain=("$(get_arg_value_from_env_value "${ZKV_CONF_CHAIN}")")
  chain_id=$("${ZKV_NODE}" build-spec --chain "${chain}" 2> /dev/null | grep \"id\": | awk -F'"' '{print $4}')
  destination="${base_path}/chains/${chain_id}/network"
  mkdir -p "${destination}"
  echo "Copying node key file"
  cp "${ZKV_NODE_KEY_FILE}" "${destination}/secret_ed25519"
fi

echo "Launching ${ZKV_NODE} with args ${conf_args[*]}"
exec "${ZKV_NODE}" "${conf_args[@]}"

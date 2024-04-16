#!/usr/bin/env bash

# This script performs the following tasks:
# 
# - translation of environment variables to command line arguments
# - preparation before the node start (example keys injection)
# - launch of the actual node
# 
# Environment variables should generally be in the form `NH_*`
# Environment variables in the form `NH_CONF_*` are translated to command line arguments based on these rules:
#
# 1. `NH_CONF_` prefix is removed
# 2. if both a trailing underscore (`_`) and number are present, they are removed
# 3. if underscores (`_`) are present, they are replaced with dashes (`-`)
# 4. letters are replaced with lower case
# 5. prefix `--` is added
# 
# Examples:
# 
# - `NH_CONF_BASE_PATH` -> `--base-path`
# - `NH_CONF_BOOTNODES` -> `--bootnodes`
# - `NH_CONF_BOOTNODES_2` -> `--bootnodes`
#
# Values of environment variables are used unmodified as values of command line arguments with the exception
# of `true` being dropped (as a flag, example `NH_CONF_VALIDATOR`/`--validator`)

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
NH_NODE="${BINARIES[0]}"
echo "NH_NODE=${NH_NODE}"

NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH:-"/data/config/secret_phrase.dat"}
echo "NH_SECRET_PHRASE_PATH=${NH_SECRET_PHRASE_PATH}"

# Node configurations (env->arg)
prefix="NH_CONF_"
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

# Keys handling
if [ -f "${NH_SECRET_PHRASE_PATH}" ]; then
  injection_args=()
  if [ -n "${NH_CONF_BASE_PATH:-}" ]; then
    injection_args+=("$(get_arg_name_from_env_name NH_CONF_BASE_PATH ${prefix})")
    injection_args+=("$(get_arg_value_from_env_value "${NH_CONF_BASE_PATH}")")
  fi
  if [ -n "${NH_CONF_CHAIN:-}" ]; then
    injection_args+=("$(get_arg_name_from_env_name NH_CONF_CHAIN ${prefix})")
    injection_args+=("$(get_arg_value_from_env_value "${NH_CONF_CHAIN}")")
  fi
  echo "Injecting keys with ${injection_args[*]}"
  echo "Injecting key (Aura)"
  ${NH_NODE} key insert "${injection_args[@]}" \
    --scheme Sr25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type aura
  echo "Injecting key (Grandpa)"
  ${NH_NODE} key insert "${injection_args[@]}" \
    --scheme Ed25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type gran
  echo "Injecting key (Imonline)"
  ${NH_NODE} key insert "${injection_args[@]}" \
    --scheme Sr25519 \
    --suri "${NH_SECRET_PHRASE_PATH}" \
    --key-type imon
fi

echo "Launching ${NH_NODE} with args ${conf_args[*]}"
exec "${NH_NODE}" "${conf_args[@]}"
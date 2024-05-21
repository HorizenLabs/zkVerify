#!/usr/bin/env bash

# SPDX-License-Identifier: Apache-2.0
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# 	http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# - Pallet benchmarking to update the pallet weights
# - Overhead benchmarking for the Extrinsic and Block weights
# - Machine benchmarking
#
# Should be run on a reference machine to gain accurate benchmarks

# shellcheck disable=SC2181

ECHO_CMD=${ECHO_CMD:="false"}

if [ "${ECHO_CMD}" == "true" ]; 
then
      set -x
fi

USE_DOCKER_IMAGE=${USE_DOCKER_IMAGE:=""}

# The following line ensure we know the project root
SOURCE_ROOT=${SOURCE_ROOT:-$(git rev-parse --show-toplevel)}

. "${SOURCE_ROOT}/scripts/bench_cfg.sh"

BENCH_SH=${BENCH_SH:-"${SOURCE_ROOT}/scripts/bench.sh"}
# Define the error file.
ERR_FILE=${ERR_FILE:-"benchmarking_errors.txt"}

if [ -z "${USE_DOCKER_IMAGE}" ]; 
then
echo "[+] Compiling nh-node benchmarks..."
cargo build \
  --locked \
  --features=runtime-benchmarks \
  --bin nh-node

  # The executable to use.
  NH_NODE="${PROJECT_ROOT}/target/debug/nh-node"
  SKIP_LINES=2
else
  # The executable to use.
  NH_NODE="docker run -ti --rm -v ${PROJECT_ROOT}:/data/benchmark --entrypoint /usr/local/bin/nh-node ${USE_DOCKER_IMAGE}"

  # Now PROJECT_ROOT become the docker folder
  PROJECT_ROOT="/data/benchmark"
  SKIP_LINES=4
fi

DEFAULT_DEPLOY_WEIGHT_TEMPLATE="${PROJECT_ROOT}/node/hl-deploy-weight-template.hbs"

WEIGTH_TEMPLATE=${WEIGTH_TEMPLATE:-"${DEFAULT_DEPLOY_WEIGHT_TEMPLATE}"}
WEIGHTS_FOLDER=${WEIGHTS_FOLDER:-"${PROJECT_ROOT}/runtime/src/weights"}

CODE_HEADER="${PROJECT_ROOT}/HEADER-APACHE2"



# Load all pallet names in an array.
mapfile -t < <(${NH_NODE} benchmark pallet \
  --list \
  --chain=dev | \
  tail -n+${SKIP_LINES} | \
  cut -d',' -f1 | \
  sort | \
  uniq \
  )
PALLETS=("${MAPFILE[@]}")

EXCLUDED_PALLETS=(
        # Helper pallets
        "pallet_election_provider_support_benchmarking"
        "frame_benchmarking"
        # Pallets without automatic benchmarking
        "pallet_babe" "pallet_grandpa"
        "pallet_offences"

        # Not applicable now
        "pallet_session" # Crash [to investigate]
        "pallet_staking" # Not applicable if we didn't use pallet_bag_list for VoterList and TargetList
                         # UseNominatorsAndValidatorsMap and UseValidatorsMap dosn't implement benchmark 
                         # support
        # SLOW 
        # "pallet_im_online" "frame_benchmarking" "frame_system" "pallet_balances"
    )

echo "[+] Benchmarking ${#PALLETS[@]} zkVerify pallets. (IGNORE SET [${#EXCLUDED_PALLETS[@]}])"


is_pallet_excluded() {
  local pallet=$1;

  for exluded in "${EXCLUDED_PALLETS[@]}"; do
      if [ "${exluded}" == "${pallet}" ]; then
          return 0
      fi
  done
  
  return 1
}

# Delete the error file before each run.
rm -f "${ERR_FILE}"

# Benchmark each pallet.
for PALLET in "${PALLETS[@]}"; do
  if is_pallet_excluded "${PALLET}"; then
    echo "[+] Skipping $PALLET"
    continue
  fi

  PALLET_NAME="$(echo "${PALLET}" | tr '_' '-')"
  MODULE_NAME="${PALLET}.rs";
  WEIGHT_FILE="${WEIGHTS_FOLDER}/${MODULE_NAME}"
  echo "[+] Benchmarking $PALLET with weight file $WEIGHT_FILE";

  OUTPUT=$( \
    SOURCE_ROOT="${SOURCE_ROOT}" \
    WEIGTH_OUT_PATH="${WEIGHT_FILE}" \
    SKIP_BUILD="true" \
    NH_NODE_EXE="${NH_NODE}" \
    WEIGTH_TEMPLATE="${WEIGTH_TEMPLATE}" \
    CODE_HEADER="${CODE_HEADER}" \
    BM_STEPS="${BM_STEPS}" \
    BM_REPEAT="${BM_REPEAT}" \
    BM_HEAP_PAGES="${BM_HEAP_PAGES}" \
    "${BENCH_SH}" "${PALLET_NAME}" 2>&1
  )

  if [ $? -ne 0 ]; then
    echo "$OUTPUT" >> "$ERR_FILE"
    echo "[-] Failed to benchmark $PALLET. Error written to $ERR_FILE; continuing..."
  fi
done

# Update the block and extrinsic overhead weights.
echo "[+] Benchmarking block and extrinsic overheads..."
OUTPUT=$(
  ${NH_NODE} benchmark overhead \
  --chain=dev \
  --weight-path="${WEIGHTS_FOLDER}" \
  --header="${CODE_HEADER}" \
  --warmup=10 \
  --repeat=100 2>&1
)

if [ $? -ne 0 ]; then
  echo "$OUTPUT" >> "$ERR_FILE"
  echo "[-] Failed to benchmark the block and extrinsic overheads. Error written to $ERR_FILE; continuing..."
fi

echo "[+] Benchmarking the machine..."
OUTPUT=$(
  ${NH_NODE} benchmark machine --chain=dev 2>&1
)
if [ $? -ne 0 ]; then
  # Do not write the error to the error file since it is not a benchmarking error.
  echo "[-] Failed the machine benchmark:
$OUTPUT"
fi

# Check if the error file exists.
if [ -f "$ERR_FILE" ]; then
  echo "[-] Some benchmarks failed. See: $ERR_FILE"
  exit 1
else
  echo "[+] All benchmarks passed."
  exit 0
fi

#!/bin/bash

DEFAULT_WEIGHT_TEMPLATE="node/frame-weight-template.hbs"

PALLET=$1
WEIGTH_TEMPLATE=${WEIGTH_TEMPLATE:-"${DEFAULT_WEIGHT_TEMPLATE}"}
WEIGTH_OUT_PATH=${WEIGTH_OUT_PATH:-""}

function usage {
    local message=${1:-""};

    echo "$0 <pallet> : get pallet crate name, execute benchamark and save the weigth file.
    Environment:
    WEIGTH_TEMPLATE : the template file path to use for rendering [${DEFAULT_WEIGHT_TEMPLATE}]
    WEIGTH_OUT_PATH : the path of the rendered weight file. If empty it will use <pallet_path>/src/weight.rs.
    "
    if [ -n "${message}" ]; 
    then
        echo "ERROR: $message"
    fi
    exit 1
}

if ! cargo --list | grep -q -P "^\s+workspaces$" ; 
then 
    usage "You need cargo-workspaces installed -> cargo install cargo-workspaces"
fi

if [ -z "${PALLET}" ] ;
then
    usage
fi

if ! cargo workspaces list -l -a | grep -q -w "${PALLET}" ;
then 
    usage "Pallet '${PALLET}' not found"
fi

if [ -z "${WEIGTH_OUT_PATH}" ];
then
    CRATE_PATH=$(cargo workspaces list -l -a | grep -w  "${PALLET}" | awk '{print $3 }')
    
    WEIGTH_OUT_PATH="${CRATE_PATH}/src/weight.rs"
fi

echo "------------------------------------------------------------------
Use:
PALLET=${PALLET}
WEIGTH_OUT_PATH=${WEIGTH_OUT_PATH}
WEIGTH_TEMPLATE=${WEIGTH_TEMPLATE}
------------------------------------------------------------------"

cargo build \
    --profile production \
    --features runtime-benchmarks || exit 1

./target/production/nh-node \
    benchmark pallet \
    --chain dev \
    --pallet "${PALLET}" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 30 \
    --output "${WEIGTH_OUT_PATH}" \
    --template "${WEIGTH_TEMPLATE}"

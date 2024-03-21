#!/bin/bash
# This script is used to run the e2e tests locally or in the CI pipeline.
# If runned locally, be sure that the following applications are present on
# the target system:
# - node
# - npm
# - yarn
# The script automatically downloads zombienet binary and save it into the e2e-tests/bin folder.
# It also copies there the nh-node binary, hence make sure to properly have a freshly compiled
# version in target/release or target/debug.

# ANSI color handles
TXT_BICYA="\033[96;1m"
TXT_BIPRP="\033[95;1m"
TXT_BIBLU="\033[94;1m"
TXT_BIYLW="\033[93;1m"
TXT_BIGRN="\033[92;1m"
TXT_BIRED="\033[91;1m"
TXT_BIBLK="\033[90;1m"
TXT_NORML="\033[0m"

# Please do not exceed 64 chars for each test filename - including the .zndsl extension
TEST_LIST=(
    '0001-simple_test.zndsl'
    '0002-custom_script.zndsl'
    '0003-transaction.zndsl'
    '0004-failing_transaction.zndsl'
    '0005-proofPath_rpc.zndsl'
);

# The return value of each zombinet invocation is always equal to the
# number of failed tests among those listed in each .zndsl.
# For this reason, we keep track of each .zndsl whose return value is not 0.
FAILED_TESTS=()
TOT_EXEC_TESTS=0
TOT_FAIL_TESTS=0

# Check if zombienet executable exists, otherwise download that from Parity Tech repo
if [ ! -f bin/zombienet-linux-x64 ]; then
    echo -e "${TXT_BIYLW}WARNING: ${TXT_BIBLK}Zombienet executable not found${TXT_NORML}"
    wget --progress=dot:giga https://github.com/paritytech/zombienet/releases/download/v1.3.94/zombienet-linux-x64 -O bin/zombienet-linux-x64
    if [ $? -ne 0 ]; then
        echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}zombienet binary download failed.${TXT_NORML}"
        exit 2
    fi
    chmod +x bin/zombienet-linux-x64
fi

# Check if nh-node executable exists, and copy from target/release or target/debug directory
if [ ! -f bin/nh-node ]; then
    echo -e "${TXT_BIYLW}WARNING: ${TXT_BIBLK}nh-node executable not found in e2e-tests directory${TXT_NORML}"
    if [ -f ../target/release/nh-node ]; then
        echo -e "${TXT_BIGRN}INFO: ${TXT_BIBLK}Copying release version from target/release directory${TXT_NORML}"
        cp ../target/release/nh-node bin/.
    elif [ -f ../target/debug/nh-node ]; then
        echo -e "${TXT_BIGRN}INFO: ${TXT_BIBLK}Copying debug version from target/debug directory${TXT_NORML}"
        cp ../target/debug/nh-node bin/.
    else
        echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}nh-node binary not found. Please compile NH-core before re-running this script.${TXT_NORML}"
        exit 3
    fi
fi

for TESTNAME in ${TEST_LIST[@]}; do
    echo -e "\n\n"
    echo -e "============================================================"
    echo -e ${TXT_BIBLK} "Running test: " ${TXT_NORML} "${TESTNAME}"
    echo -e "============================================================"
    bin/zombienet-linux-x64 -p native test ./${TESTNAME}
    current_exit_code=$?
    TOT_EXEC_TESTS=$((TOT_EXEC_TESTS+1))
    if [ ${current_exit_code} -ne 0 ]; then
        FAILED_TESTS+=($TESTNAME)
        TOT_FAIL_TESTS=$((TOT_FAIL_TESTS+1))
    fi
done


# Print a fancy table summarizing the test suit run
echo -e "\n\n\n"
echo -e "┌────────────────────────────────────────────────────────────────────────┐"
echo -e "│                              "${TXT_BIYLW}"TEST SUMMARY"${TXT_NORML}"                              │"
echo -e "├────────────────────────────────────────────────────────────────────────┤"
printf  "│ ${TXT_BIBLK} Total tests executed:  ${TXT_BIBLU} %3d ${TXT_NORML}                                          │\n" "${TOT_EXEC_TESTS}"
if [ ${TOT_FAIL_TESTS} -ne 0 ]; then
    echo -e "├────────────────────────────────────────────────────────────────────────┤"
    printf  "│ ${TXT_BIBLK} Failed tests:          ${TXT_BIRED} %3d ${TXT_NORML}                                          │\n" "${TOT_FAIL_TESTS}"
    for failed_test in ${FAILED_TESTS[@]}; do
        printf "│     - %-64s │\n" "${failed_test}"
    done
    echo -e "└────────────────────────────────────────────────────────────────────────┘"
    exit 1
fi
echo -e "└────────────────────────────────────────────────────────────────────────┘"
exit 0
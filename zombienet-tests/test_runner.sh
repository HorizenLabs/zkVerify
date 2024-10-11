#!/bin/bash
# This script is used to run the zombienet tests locally or in the CI pipeline.
# If run locally, be sure that the following applications are present on
# the target system:
# - node
# - npm
# - yarn
# The script automatically downloads zombienet binary and saves it into the zombienet-tests/bin folder.
# It also looks for a compiled zkv-node binary in the folder target/release, hence make sure to 
# have a freshly compiled version of zkv-node in this folder.
# Optionally, this script can be launched with the '--debug' switch, which makes it look for
# the zkv-node binary in the target/debug folder instead.

# ANSI color handles
TXT_BIBLU="\033[94;1m"
TXT_BIYLW="\033[93;1m"
TXT_BIGRN="\033[92;1m"
TXT_BIRED="\033[91;1m"
TXT_BIBLK="\033[90;1m"
TXT_NORML="\033[0m"

# The return value of each zombienet invocation is always equal to the
# number of failed tests among those listed in each .zndsl.
# For this reason, we keep track of each .zndsl whose return value is not 0.
FAILED_TESTS=()
TOT_EXEC_TESTS=0
TOT_FAIL_TESTS=0
EXIT_STATUS=0

# Check operating system and set variables for binary name
OS="$(uname)"
BASE_URL="https://github.com/paritytech/zombienet/releases/download/v1.3.94"
if [ "$OS" == "Linux" ]; then
    ZOMBIENET_BINARY="zombienet-linux-x64"
elif [ "$OS" == "Darwin" ]; then
    ZOMBIENET_BINARY="zombienet-macos"
else
    echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}Unsupported operating system.${TXT_NORML}"
    exit 4
fi

ZOMBIENET_URL="${BASE_URL}/${ZOMBIENET_BINARY}"

# Check if Zombienet executable exists, otherwise download it
if [ ! -f "bin/$ZOMBIENET_BINARY" ]; then
    mkdir -p bin
    echo -e "${TXT_BIYLW}WARNING: ${TXT_BIBLK}Zombienet executable not found${TXT_NORML}"
    curl -s -L $ZOMBIENET_URL -o "bin/$ZOMBIENET_BINARY"
    if [ $? -ne 0 ]; then
        echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}Failed to download Zombienet binary.${TXT_NORML}"
        exit 2
    fi
    chmod +x "bin/$ZOMBIENET_BINARY"
fi

declare -a TEST_LIST=()

# Check if we requested a run over a debug build
BUILDSUBPATH="release"
for ARG in "$@"; do
    if [[ "${ARG}" == "--debug" ]]; then
        BUILDSUBPATH="debug"
    else
        TEST_LIST+=("${ARG}")    
    fi
done

if [ ${#TEST_LIST[@]} -eq 0 ]; then
    # Please do not exceed 64 chars for each test filename - including the .zndsl extension
    IFS=$'\n' TEST_LIST=($(find . -name "*.zndsl" | sort))
fi

echo -e "${TXT_BIGRN}INFO: ${TXT_BIBLK}Running tests with a ${BUILDSUBPATH} build${TXT_NORML}"

# Check if zkv-node executable exists according to the requested mode and print error/info messages otherwise
if [[ ${BUILDSUBPATH} == "debug" ]]; then
    if [ ! -f ../target/debug/zkv-node ]; then
        if [ -f ../target/release/zkv-node ]; then
            echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}debug binary not found; however, a release binary is present. Compile zkv-node in debug mode${TXT_NORML}"
            echo -e "${TXT_BIRED}       ${TXT_BIBLK}or relaunch the test runner without the '--debug' switch${TXT_NORML}"
            exit 2
        else
            echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}zkv-node binary not found. Compile zkv-node in debug mode and re-launch this script${TXT_NORML}"
            exit 3
        fi
    fi
fi

if [[ ${BUILDSUBPATH} == "release" ]]; then
    if [ ! -f ../target/release/zkv-node ]; then
        if [ -f ../target/debug/zkv-node ]; then
            echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}release binary not found; however, a debug binary is present. Compile zkv-node in release mode${TXT_NORML}"
            echo -e "${TXT_BIRED}       ${TXT_BIBLK}or relaunch the test runner with the '--debug' switch${TXT_NORML}"
            exit 2
        else
            echo -e "${TXT_BIRED}ERROR: ${TXT_BIBLK}zkv-node binary not found. Compile zkv-node in release mode and re-launch this script${TXT_NORML}"
            exit 3
        fi
    fi
fi

# If all checks passed, set the full build path
FULLBUILDPATH="../target/${BUILDSUBPATH}"

# GO! GO! GO!
for TESTNAME in "${TEST_LIST[@]}"; do
    echo -e "\n\n"
    echo -e "============================================================"
    echo -e "${TXT_BIBLK} Running test:  ${TXT_NORML} ${TESTNAME}"
    echo -e "============================================================"
    ( PATH=${PATH}:${FULLBUILDPATH}; bin/$ZOMBIENET_BINARY -p native test ./"${TESTNAME}" )
    current_exit_code=$?
    TOT_EXEC_TESTS=$((TOT_EXEC_TESTS+1))
    if [ ${current_exit_code} -ne 0 ]; then
        FAILED_TESTS+=("$TESTNAME")
        TOT_FAIL_TESTS=$((TOT_FAIL_TESTS+1))
        EXIT_STATUS=1
    fi
done

# Print a fancy table summarizing the test suit run
echo -e "\n\n\n"
echo -e "┌────────────────────────────────────────────────────────────────────────┐"
echo -e "│                              ${TXT_BIYLW}TEST SUMMARY${TXT_NORML}                              │"
echo -e "├────────────────────────────────────────────────────────────────────────┤"
printf  "│ ${TXT_BIBLK} Total tests executed:  ${TXT_BIBLU} %3d ${TXT_NORML}                                          │\n" "${TOT_EXEC_TESTS}"
PASSED_TESTS=$((TOT_EXEC_TESTS - TOT_FAIL_TESTS))
printf  "│ ${TXT_BIBLK} Passed tests:          ${TXT_BIGRN} %3d ${TXT_NORML}                                          │\n" "${PASSED_TESTS}"
printf  "│ ${TXT_BIBLK} Failed tests:          ${TXT_BIRED} %3d ${TXT_NORML}                                          │\n" "${TOT_FAIL_TESTS}"

if [ ${TOT_FAIL_TESTS} -ne 0 ]; then
    echo -e "├────────────────────────────────────────────────────────────────────────┤"
    for failed_test in "${FAILED_TESTS[@]}"; do
        printf "│     - %-64s │\n" "${failed_test}"
    done
fi
echo -e "└────────────────────────────────────────────────────────────────────────┘"

exit ${EXIT_STATUS}

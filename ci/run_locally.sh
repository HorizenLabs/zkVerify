#!/bin/bash
set -eEuo pipefail

root_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &>/dev/null && pwd)"
common_file_location="${root_dir}/ci/common.sh"
workflows_dir="${root_dir}/.github/workflows"
PRE_PUSH_HOOK="${PRE_PUSH_HOOK:-false}"


####
# Checking all the requirement(s)
####
if ! [ -f "${common_file_location}" ]; then
  echo -e "\n\033[1;31mERROR: ${common_file_location} file is missing !!!  Exiting ...\033[0m\n"
  exit 1
else
  # shellcheck disable=SC1090
  source "${common_file_location}"
fi
check_requirements

# Starting from fresh act environment
act_dangling_containers="$(docker ps -a -q -f "name=act")" || fn_die "ERROR: Failed to execute the 'docker ps' command. Please ensure the 'docker' service is running. Exiting ..."
if [ -n "${act_dangling_containers}" ]; then
  log_info "\n=== Cleaning up dangling 'act' containers ==="
  docker rm -f "${act_dangling_containers}" || fn_die "ERROR: Failed to execute the 'docker rm -f' command. Please ensure the 'docker' service is running. Exiting ..."
fi


####
# Running workflow(s)
####
#workflows="CI-build-test CI-coverage CI-lint-format CI-e2e-test"
workflows_orchestrator="CI-build-test CI-coverage CI-lint-format CI-e2e-test"
extra_workflows="CI-rustdoc"
if [ "${PRE_PUSH_HOOK}" == 'false' ];then
  workflows="${workflows_orchestrator} ${extra_workflows}"
  while true; do
    # choose one of the available workflows
    log_warn "\nPlease select a workflow to run:"
    workflow="$(selection "${workflows}")" || fn_die "ERROR: Failed to execute 'selection' function for listing workflows to run. Exiting ..."

    if [ "${workflow}" == 'QUIT' ]; then
      break
    else
      log_debug "\n=== Running ${workflows_dir}/${workflow}.yml workflow ==="
      act --detect-event --rm -W "${workflows_dir}/${workflow}.yml" || fn_die "ERROR: attempt to run ${workflows_dir}/${workflow}.yml workflow locally has failed. Exiting ..."
    fi
  done
elif [ "${PRE_PUSH_HOOK}" == 'true' ]; then
  #workflows="CI-rustdoc"
  for workflow in ${workflows_orchestrator}; do
    log_debug "\n=== Running ${workflows_dir}/${workflow}.yml workflow ==="
    act --detect-event --rm -W "${workflows_dir}/${workflow}.yml" || fn_die "ERROR: attempt to run ${workflows_dir}/${workflow}.yml workflow locally has failed. Exiting ..."
  done
fi


####
# End
####
log_info "\n=== Done ==="
exit 0
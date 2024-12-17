#!/bin/bash
set -eEo pipefail

export IS_A_RELEASE="false"
export PROD_RELEASE="false"
export DEV_RELEASE="false"
export TEST_RELEASE="false"
export COMMON_FILE_LOCATION='ci/common.sh'

workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
github_tag="${GITHUB_TAG:-refs/tags/0.7.0-0.9.0-test2}"
release_branch="${RELEASE_BRANCH:-release}"
prod_release_regex='^[0-9]+\.[0-9]+\.[0-9]+\-[0-9]+\.[0-9]+\.[0-9]+$'
dev_release_regex='^[0-9]+\.[0-9]+\.[0-9]+\-[0-9]+\.[0-9]+\.[0-9]+(-rc[0-9]+){1}$'
test_release_regex='^[0-9]+\.[0-9]+\.[0-9]+\-[0-9]+\.[0-9]+\.[0-9]+-[a-zA-Z0-9]+$'

source ci/common.sh

####
# Main
####
log_info "Release branch(es) is(are): ${release_branch}/*"
log_info "Github tag is: ${github_tag}"

# Checking if it is a release build
if git branch -r --contains "${github_tag}" | grep -xqE ". origin\/${release_branch}/${github_tag}"; then
  IS_A_RELEASE="true"
  derived_from_branch="$(git branch -r --contains "${github_tag}" | grep -xE ". origin\/${release_branch}/${github_tag}")"

  # if [ -z "${MAINTAINERS_KEYS:-}" ]; then
  #   log_warn "WARNING: MAINTAINERS_KEYS variable is not set. The build is not going to be released ..."
  # fi

  # import_gpg_keys "${MAINTAINERS_KEYS}"
  # check_signed_tag "${github_tag}"

  # Release test
  if [ "${IS_A_RELEASE}" = "true" ]; then
    if [[ "${github_tag}" =~ ${prod_release_regex} ]]; then
      export PROD_RELEASE="true"
    elif [[ "${github_tag}" =~ ${dev_release_regex} ]]; then
      export DEV_RELEASE="true"
    elif [[ "${github_tag}" =~ ${test_release_regex} ]] && ! [[ "${github_tag}" =~ -rc ]]; then
      export TEST_RELEASE="true"
    else
      log_warn "WARNING: GitHub tag: ${github_tag} is in the wrong format for PRODUCTION, DEVELOPMENT or TEST release. Expecting the following format for the release: PRODUCTION = 'd.d.d-d.d.d' | DEVELOPMENT = 'd.d.d-d.d.d-rc[0-9]' | TEST = 'd.d.d-d.d.d-*'. The build is not going to be released ..."
      export IS_A_RELEASE="false"
    fi
  fi
else
  log_warn "WARNING: GitHub tag = ${github_tag} does NOT derive from any '${release_branch}/*' branches. The build is not going to be released ..."
fi

# Final check for release vs non-release build
if [ "${PROD_RELEASE}" = "true" ]; then
  echo "" && log_info "=== This is a Production release build ===" && echo ""
elif [ "${DEV_RELEASE}" = "true" ]; then
  echo "" && log_info "=== This is a Development release build ===" && echo ""
elif [ "${TEST_RELEASE}" = "true" ]; then
  echo "" && log_info "=== This is a Test release build ===" && echo ""
elif [ "${IS_A_RELEASE}" = "false" ]; then
  echo "" && log_warn "WARNING: This is NOT a RELEASE build" && echo ""
fi

set +eo pipefail

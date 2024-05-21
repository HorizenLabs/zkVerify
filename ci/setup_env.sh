#!/bin/bash
set -eEo pipefail

export IS_A_RELEASE="false"
export PROD_RELEASE="false"
export DEV_RELEASE="false"
export TEST_RELEASE="false"
export COMMON_FILE_LOCATION='ci/common.sh'

workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
github_tag="${GITHUB_REF_NAME:-}"
release_branch="${RELEASE_BRANCH:-release}"
prod_release_regex='^[0-9]+\.[0-9]+\.[0-9]+$'
dev_release_regex='^[0-9]+\.[0-9]+\.[0-9]+(-rc[0-9]+){1}$'
test_release_regex='^[0-9]+\.[0-9]+\.[0-9]+-[a-zA-Z0-9]+$'

# Requirement
if ! [ -f "${workdir}/${COMMON_FILE_LOCATION}" ]; then
  echo -e "\n\033[1;31mERROR: ${COMMON_FILE_LOCATION} file is missing !!! \033[0m\n"
  return
else
  # shellcheck disable=SC1090
  source "${COMMON_FILE_LOCATION}"
fi

# Functions
import_gpg_keys() {
  # shellcheck disable=SC2207
  declare -r my_arr=( $(echo "${@}" | tr " " "\n") )

  if [ "${#my_arr[@]}" -eq 0 ]; then
    log bold yellow "WARNING: there are ZERO gpg keys to import. Please check if MAINTAINERS_KEYS variable(s) is(are) set correctly. The build is not going to be released ..."
    export IS_A_RELEASE="false"
  else
    # shellcheck disable=SC2145
    printf "%s\n" "Tagged build, fetching keys:" "${@}" ""
    for key in "${my_arr[@]}"; do
      gpg -v --batch --keyserver hkps://keys.openpgp.org --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://keyserver.ubuntu.com --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://pgp.mit.edu:80 --recv-keys "${key}" ||
      gpg -v --batch --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys "${key}" ||

      { log bold yellow "WARNING: ${key} can not be found on GPG key servers. Please upload it to at least one of the following GPG key servers:\nhttps://keys.openpgp.org/\nhttps://keyserver.ubuntu.com/\nhttps://pgp.mit.edu/"; export IS_A_RELEASE="false"; }
    done
  fi
}

check_signed_tag() {
  local tag="${1}"

  if git verify-tag -v "${tag}"; then
    echo "${tag} is a valid signed tag"
  else
    log bold yellow "WARNING: GIT's tag = ${tag} signature is NOT valid. The build is not going to be released ..."
    export IS_A_RELEASE="false"
  fi
}


####
# Main
####
log italic green "Release branches are: ${release_branch}/*"
log italic green "Github tag is: ${github_tag}"

# Checking if it is a release build
if git branch -r --contains "${github_tag}" | grep -xqE ". origin\/${release_branch}/[^/]+$"; then
  IS_A_RELEASE="true"
  derived_from_branch="$(git branch -r --contains "${github_tag}" | grep -xE ". origin\/${release_branch}/[^/]+$")"
  release_br_amount="$(wc -l <<< "${derived_from_branch}")"
  # Sanity check
  if [ "${release_br_amount}" -ne 1 ]; then
    log bold yellow "WARNING: More than 1 GitHub '${release_branch}/*' branch contains current GitHub tag: ${github_tag}. The build is not going to be released ..."
    IS_A_RELEASE="false"
  fi

  if [ -z "${MAINTAINERS_KEYS:-}" ]; then
    log bold yellow "WARNING: MAINTAINERS_KEYS variable is not set. The build is not going to be released ..."
  fi

  import_gpg_keys "${MAINTAINERS_KEYS}"
  check_signed_tag "${github_tag}"

  # Release test
  if [ "${IS_A_RELEASE}" = "true" ]; then
    # Checking if github tag was created from release/* (release/1.1.1 and etc) branch
    release_name="$(cut -d '/' -f3 <<< "${derived_from_branch}")"
    # Checking if branch name after 'release/' matches github tag name
    if [ "${release_name}" = "${github_tag}" ]; then
      if [[ "${github_tag}" =~ ${prod_release_regex} ]]; then
        export PROD_RELEASE="true"
      elif [[ "${github_tag}" =~ ${dev_release_regex} ]]; then
        export DEV_RELEASE="true"
      elif [[ "${github_tag}" =~ ${test_release_regex} ]] && ! [[ "${github_tag}" =~ -rc ]]; then
        export TEST_RELEASE="true"
      else
        log bold yellow "WARNING: GitHub tag: ${github_tag} is in the wrong format for PRODUCTION, DEVELOPMENT or TEST release. Expecting the following format for the release: PRODUCTION = 'd.d.d' | DEVELOPMENT = 'd.d.d-rc[0-9]' | TEST = 'd.d.d-*'. The build is not going to be released ..."
        export IS_A_RELEASE="false"
      fi
    else
      log bold yellow "WARNING: GitHub tag = ${github_tag} does NOT match GitHub release branch name = ${release_name}. The build is not going to be released ..."
      export IS_A_RELEASE="false"
    fi
  fi
else
  log bold yellow "WARNING: GitHub tag = ${github_tag} does NOT derive from any '${release_branch}/*' branches. The build is not going to be released ..."
fi

# Final check for release vs non-release build
if [ "${PROD_RELEASE}" = "true" ]; then
  echo "" && log bold green "=== This is a Production release build ===" && echo ""
elif [ "${DEV_RELEASE}" = "true" ]; then
  echo "" && log bold green "=== This is a Development release build ===" && echo ""
elif [ "${TEST_RELEASE}" = "true" ]; then
  echo "" && log bold green "=== This is a Test release build ===" && echo ""
elif [ "${IS_A_RELEASE}" = "false" ]; then
  echo "" && log bold yellow "WARNING: This is NOT a RELEASE build" && echo ""
fi

set +eo pipefail
exit 1

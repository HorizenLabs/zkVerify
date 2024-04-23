#!/bin/bash
set -eEuo pipefail

workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
docker_image_build_name="${DOCKER_IMAGE_BUILD_NAME:-nh-node}"
docker_hub_org="${DOCKER_HUB_ORG:-horizenlabs}"
docker_hub_username="${DOCKER_HUB_USERNAME:-}"
docker_hub_token="${DOCKER_HUB_TOKEN:-}"
is_a_release="${IS_A_RELEASE:-false}"
prod_release="${PROD_RELEASE:-false}"
dev_release="${DEV_RELEASE:-false}"
test_release="${TEST_RELEASE:-false}"
github_ref_name="${GITHUB_REF_NAME:-}"
common_file_location="${COMMON_FILE_LOCATION:-not-set}"
docker_file_path='docker/dockerfiles/hl-node.Dockerfile'

# Requirement
if ! [ -f "${common_file_location}" ]; then
  echo -e "\n\033[1;31mERROR: ${common_file_location} file is missing !!!  Exiting ...\033[0m\n"
  exit 1
else
  # shellcheck disable=SC1090
  source "${common_file_location}"
fi


####
# Main
####
if [ -z "${docker_hub_token:-}" ]; then
  fn_die "ERROR: DOCKER_HUB_TOKEN variable is not set. Exiting ..."
fi

if [ -z "${docker_hub_username:-}" ]; then
  fn_die "ERROR: DOCKER_HUB_USERNAME variable is not set. Exiting ..."
fi

docker_tag=""
if [ "${is_a_release}" = "true" ]; then
  docker_tag="${github_ref_name}"
fi

# Building and publishing docker image
if [ -n "${docker_tag:-}" ]; then
  log italic green "=== Building Docker image: ${docker_hub_org}/${docker_image_build_name}:${docker_tag} ==="
  docker build -f "${docker_file_path}" -t "${docker_hub_org}/${docker_image_build_name}:${docker_tag}" .

  # Publishing to DockerHub
  log italic green "=== Publishing Docker image(s) on Docker Hub ==="
  echo "${docker_hub_token}" | docker login -u "${docker_hub_username}" --password-stdin

  # Docker image(s) tags for PROD vs DEV release
  if [ "${prod_release}" = "true" ]; then
    publish_tags=("${docker_tag}" "latest")
  elif [ "${dev_release}" = "true" ]; then
    publish_tags=("${docker_tag}" "dev")
  elif [ "${test_release}" = "true" ]; then
    publish_tags=("${docker_tag}")
  fi

  for publish_tag in "${publish_tags[@]}"; do
    log italic green "Publishing docker image: ${docker_image_build_name}:${publish_tag}"
    docker tag "${docker_hub_org}/${docker_image_build_name}:${docker_tag}" "index.docker.io/${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
    docker push "index.docker.io/${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
  done
else
  fn_die "ERROR: the build did NOT satisfy RELEASE build requirements. Docker image(s) was(were) NOT build and/or published."
fi

exit 0
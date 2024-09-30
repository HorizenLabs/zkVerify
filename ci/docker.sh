#!/bin/bash
set -eEuo pipefail

workdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." &> /dev/null && pwd )"
docker_image_build_name="${DOCKER_IMAGE_BUILD_NAME:-zkverify}"
docker_hub_org="${DOCKER_HUB_ORG:-horizenlabs}"
docker_hub_username="${DOCKER_HUB_USERNAME:-}"
docker_hub_token="${DOCKER_HUB_TOKEN:-}"
is_a_release="${IS_A_RELEASE:-false}"
prod_release="${PROD_RELEASE:-false}"
dev_release="${DEV_RELEASE:-false}"
test_release="${TEST_RELEASE:-false}"
github_ref_name="${GITHUB_REF_NAME:-}"
common_file_location="${COMMON_FILE_LOCATION:-not-set}"
docker_file_path='docker/dockerfiles/zkv-node.Dockerfile'

private_docker_repo="${PRIVATE_DOCKER_REPO:-}"
commit_hash="${COMMIT_HASH:-}"

# Requirement
if ! [[ -f "${common_file_location}" ]]; then
  echo -e "\n\033[1;31mERROR: ${common_file_location} file is missing !!! Exiting ...\033[0m\n"
  exit 1
else
  # shellcheck disable=SC1090
  source "${common_file_location}"
fi


####
# Main
####
cd "${workdir}"

if [[ -z "${docker_hub_token:-}" ]]; then
  fn_die "ERROR: DOCKER_HUB_TOKEN variable is not set. Exiting ..."
fi

if [[ -z "${docker_hub_username:-}" ]]; then
  fn_die "ERROR: DOCKER_HUB_USERNAME variable is not set. Exiting ..."
fi

docker_tag_full=""
publish_tags=()

if [[ "${is_a_release}" == "true" ]]; then
  docker_tag_full="${github_ref_name}"

  # Determine image tags based on release type
  if [[ "${prod_release}" == "true" ]]; then
    docker_tag_node="$(cut -d '-' -f1 <<< "${docker_tag_full}")"
    publish_tags=("${docker_tag_full}" "${docker_tag_node}" "latest")
  elif [[ "${dev_release}" == "true" ]]; then
    docker_tag_node="$(cut -d '-' -f1 <<< "${docker_tag_full}")-$(cut -d '-' -f3- <<< "${docker_tag_full}")"
    publish_tags=("${docker_tag_full}" "${docker_tag_node}")
  elif [[ "${test_release}" == "true" ]]; then
    publish_tags=("${docker_tag_full}")
  else
    fn_die "ERROR: No valid release type specified for IS_A_RELEASE=true."
  fi

  # Build and publish images to DockerHub (and private repo if set)
  # Build the Docker image
  log_info "=== Building Docker image: ${docker_image_build_name}:${docker_tag_full} ==="
  docker build --build-arg PROFILE=production -f "${docker_file_path}" -t "${docker_image_build_name}:${docker_tag_full}" .

  # Login to DockerHub
  log_info "=== Publishing Docker image(s) on Docker Hub ==="
  echo "${docker_hub_token}" | docker login -u "${docker_hub_username}" --password-stdin

  # Tag and push to DockerHub
  for publish_tag in "${publish_tags[@]}"; do
    log_info "Publishing docker image to Docker Hub: ${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
    docker tag "${docker_image_build_name}:${docker_tag_full}" "${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
    docker push "${docker_hub_org}/${docker_image_build_name}:${publish_tag}"
  done

  # Publish to Private Repository if specified
  if [[ -n "${private_docker_repo:-}" ]]; then
    log_info "=== Publishing Docker image(s) to Private Repository: ${private_docker_repo} ==="
    if [[ -z "${DOCKER_HUB_TOKEN:-}" || -z "${DOCKER_HUB_USERNAME:-}" ]]; then
      fn_die "ERROR: DOCKER_HUB_USERNAME and DOCKER_HUB_TOKEN must be set for private repository operations."
    fi
    echo "${DOCKER_HUB_TOKEN}" | docker login -u "${DOCKER_HUB_USERNAME}" --password-stdin "${private_docker_repo}"

    for publish_tag in "${publish_tags[@]}"; do
      log_info "Publishing docker image to Private Repository: ${private_docker_repo}:${publish_tag}"
      docker tag "${docker_image_build_name}:${docker_tag_full}" "${private_docker_repo}:${publish_tag}"
      docker push "${private_docker_repo}:${publish_tag}"
    done
  fi

elif [[ "${is_a_release}" == "false" && "${dev_release}" == "true" && -n "${private_docker_repo:-}" ]]; then
  # IS_A_RELEASE is false, but PRIVATE_DOCKER_REPO is set and DEV_RELEASE is true
  if [[ -n "${commit_hash:-}" ]]; then
    docker_tag_full="${commit_hash}"
  else
    docker_tag_full="dev"
  fi
  publish_tags=("${docker_tag_full}")

  # Build and publish images to PRIVATE_DOCKER_REPO only
  # Build the Docker image
  log_info "=== Building Docker image: ${docker_image_build_name}:${docker_tag_full} ==="
  docker build --build-arg PROFILE=production -f "${docker_file_path}" -t "${docker_image_build_name}:${docker_tag_full}" .

  # Login to Private Docker Repository
  log_info "=== Publishing Docker image(s) to Private Docker Repository: ${private_docker_repo} ==="
  if [[ -z "${DOCKER_HUB_TOKEN:-}" || -z "${DOCKER_HUB_USERNAME:-}" ]]; then
    fn_die "ERROR: DOCKER_HUB_USERNAME and DOCKER_HUB_TOKEN must be set for private repository operations."
  fi
  echo "${DOCKER_HUB_TOKEN}" | docker login -u "${DOCKER_HUB_USERNAME}" --password-stdin

  # Tag and push to Private Repository
  for publish_tag in "${publish_tags[@]}"; do
    log_info "Publishing docker image to Private Repository: ${private_docker_repo}:${publish_tag}"
    docker tag "${docker_image_build_name}:${docker_tag_full}" "${private_docker_repo}:${publish_tag}"
    docker push "${private_docker_repo}:${publish_tag}"
  done

else
  # Do not build images
  log_warn "Build does not meet the criteria for image building. No Docker images were built or published."
fi

exit 0

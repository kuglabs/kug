#!/usr/bin/env bash
#
# Builds a buildkite pipeline based on the environment variables
#

set -e
cd "$(dirname "$0")"/..

output_file=${1:-/dev/stderr}

if [[ -n $CI_PULL_REQUEST ]]; then
  IFS=':' read -ra affected_files <<< "$(buildkite-agent meta-data get affected_files)"
  if [[ ${#affected_files[*]} -eq 0 ]]; then
    echo "Unable to determine the files affected by this PR"
    exit 1
  fi
else
  affected_files=()
fi

wait_step() {
  echo "  - wait" >> "$output_file"
}

start_pipeline "Push pipeline for ${BUILDKITE_BRANCH:-?unknown branch?}"
wait_step
exit 0

#!/bin/bash

set -euo pipefail

skip_test="${1:-}"
container_name=rust

docker exec -it "$container_name" cargo build

if [[ -z $skip_test ]]; then
  docker exec -it "$container_name" cargo test
fi

#!/bin/bash

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

set -euo pipefail

target="${1:-x86_64-unknown-linux-gnu}"
toolchain="${2:-stable}"
image_name="rust-$target"
container_name=rust

$script_dir/../docker/start_container.sh "$target" "$toolchain" "$image_name" "$container_name"

# check installation
docker exec -it "$container_name" rustup --version
docker exec -it "$container_name" rustup target list | grep 'default\|installed'
docker exec -it "$container_name" cargo --version

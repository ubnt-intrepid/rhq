#!/bin/bash

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

set -euo pipefail

main() {
  case `uname -s` in
    Linux)
      docker rm -f "$container_name" || true
      docker run --name "$container_name" -d -it --privileged -v "$(pwd)":/root/src -w /root/src "$image_name"
      install_rustup "$container_name" "$target" "$toolchain"
      ;;

    Darwin)
      curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$toolchain" --default-host "$target"
      ;;
  esac
}

install_rustup() {
  local container_name="$1"
  local target="$2"
  local toolchain="$3"

  docker exec -it "$container_name" apt-get install -y curl
  curl -sSf https://sh.rustup.rs | docker exec -i "$container_name" sh -s -- -y --default-toolchain "$toolchain"

  if [[ "$target" = "x86_64-unknown-linux-gnu" ]]; then
    return
  fi

  docker exec -it "$container_name" /root/.cargo/bin/rustup target add "$target"
  echo -e "[build]\ntarget = \"$target\"" | docker exec -i "$container_name" tee /root/.cargo/config
  case $target in
    arm-linux-androideabi|i686-linux-android)
      echo -e "\n[target.$target]\nlinker = \"$target-gcc\"" | docker exec -i "$container_name" tee -a /root/.cargo/config ;;
  esac
}

#= PARAMETERS ===================================
target="${1:-x86_64-unknown-linux-gnu}"
toolchain="${2:-stable}"
container_name="${3:-rust}"
api="${API:-24}"

case $target in
  *arm-linux-androideabi*|*i686-linux-android*)
    image_name="ubntintrepid/${target}:api${api}"
    if [[ "$target" = "arm-linux-androideabi" ]]; then arch=arm; fi
    if [[ "$target" = "i686-linux-android"    ]]; then arch=x86; fi
    if [[ `docker images -q "$image_name" | wc -l` = 0 ]]; then
      docker build -t "$image_name" \
        --build-arg ARCH="$arch" \
        --build-arg API="$api" \
        "$script_dir/rust-android-builder"
    fi
    ;;
  *-apple-darwin)
    ;;
  *)
    image_name="japaric/${target}:latest"
    docker pull "$image_name"
    ;;
esac

main

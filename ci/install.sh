#!/bin/bash

set -euo pipefail

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

target="${1:-x86_64-unknown-linux-gnu}"
toolchain="${2:-stable}"
image_name="rust-$target"
container_name=rust

case `uname -s` in
  Linux)
    $script_dir/../docker/start_container.sh "$target" "$toolchain" "$image_name" "$container_name"

    # check installation
    docker exec -it "$container_name" rustup --version
    docker exec -it "$container_name" rustup target list | grep 'default\|installed'
    docker exec -it "$container_name" cargo --version
    ;;

  *)
    curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$toolchain"
    source $HOME/.cargo/env
    rustup target add "$target"

    default_toolchain="$(rustup target list | grep default | awk '{print $1}')"
    if [[ $default_toolchain -ne $toolchain ]]; then
      echo "[build]" > $HOME/.cargo/config
      echo "target = \"${target}\"" >> $HOME/.cargo/config
    fi

    # check installation
    rustup --version
    rustup target list | grep 'default\|installed'
    cargo --version
    ;;
esac

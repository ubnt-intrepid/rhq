#!/bin/bash

set -euo pipefail
set -x

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

target="$1"
toolchain="$2"

setup_rustup_docker() {
  docker build -t "$1" "${@:2}" $script_dir/docker/"$1"

  docker rm -f rust-container || true
  docker run --name rust-container \
    -d -it \
    -v "$(pwd)":/home/rust/src \
    -w /home/rust/src \
    --privileged \
    "$1"

  docker exec -it rust-container rustup update
  docker exec -it rust-container rustup default "$toolchain"
  case $target in
  i686-unknown-linux-musl|*-linux-androideabi|*-linux-android)
    docker exec -it rust-container rustup target install "$target"
    ;;
  esac
}


setup_rustup() {
  case $target in
  i686-unknown-linux-gnu|*-unknown-freebsd)
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain "$toolchain"
    ;;
  *)
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain "$toolchain" --default-host "$target"
    ;;
  esac
  case $target in
  i686-unknown-linux-gnu|*-unknown-freebsd)
    $HOME/.cargo/bin/rustup target add "$target" ;;
  esac
}


case $target in
*-unknown-linux-musl)
  setup_rustup_docker "rust-musl-builder"
  ;;

*-linux-androideabi|*-linux-android)
  setup_rustup_docker "rust-android-builder" \
    --build-arg TARGET=$TARGET \
    --build-arg ARCH=$ARCH \
    --build-arg API=$API
  ;;

*)
  setup_rustup
  ;;
esac

# check installation
source $HOME/.cargo/env || true
export TARGET=$target
$script_dir/rustup.sh --version
$script_dir/rustup.sh target list | grep 'default\|installed'
$script_dir/cargo.sh --version
$script_dir/rustup.sh run "$toolchain" rustc --version

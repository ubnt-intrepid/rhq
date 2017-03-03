#!/bin/bash

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

set -euo pipefail
set -x

target="$1"
toolchain="$2"
image_name="$3"
container_name="$4"

ls -F "$script_dir" | grep '/$' | sed -e 's!/$!!' | grep $target || {
  echo "error: '$target' is unsupported target"
  exit 1
}

case $target in
*-linux-androideabi|*-linux-android)
  docker build -t "$image_name" --build-arg TOOLCHAIN="$toolchain" --build-arg API="$API" "$script_dir/$target"
  ;;
*)
  docker build -t "$image_name" --build-arg TOOLCHAIN="$toolchain" "$script_dir/$target"
  ;;
esac

docker rm -f "$container_name" || true
docker run --name "$container_name" -d -it --privileged -v "$(pwd)":/root/src "$image_name"

#!/bin/bash

set -euo pipefail

if [[ -z "$TARGET" ]]; then
  echo "\$TARGET is empty."
  exit 1
fi

case $TARGET in
  *-unknown-linux-musl|*-linux-androideabi|*-linux-android)
    echo "[Use docker container]"
    docker exec -it rust-container cargo "$@"
    ;;
  *)
    echo "[Use standard toolchain]"
    cargo "$@"
    ;;
esac

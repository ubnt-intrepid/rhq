#!/bin/bash

set -euo pipefail

if [[ -z "$TARGET" ]]; then
  echo "\$TARGET is empty."
  exit 1
fi

case $TARGET in
  *-unknown-linux-musl|*-linux-androideabi|*-linux-android)
    echo "[Use docker container]"
    docker exec -it rust-container rustup "$@"
    ;;
  *)
    echo "[Use standard toolchain]"
    rustup "$@"
    ;;
esac

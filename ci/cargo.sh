#!/bin/bash

set -euo pipefail

case `uname -s` in
  Linux)
    docker exec -it "rust" /root/.cargo/bin/cargo "$@"
    ;;
  *)
    cargo "$@"
esac

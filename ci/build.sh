#!/bin/bash

set -euo pipefail

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"
skip_test="${1:-}"

$script_root/cargo.sh build

if [[ -z $skip_test ]]; then
  $script_root/cargo.sh test
fi

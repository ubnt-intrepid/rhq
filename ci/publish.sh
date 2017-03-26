#!/bin/bash

set -euo pipefail

api_key="$1"
cargo login "$api_key"

set -v
cargo package
cargo publish

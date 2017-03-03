#!/bin/bash

set -euo pipefail

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"
pkgname="$1"

$script_dir/cargo.sh build --release

rm -rf ./"${pkgname}"
mkdir -p ./"${pkgname}"
cp ./target/release/rhq ./"${pkgname}"/

cd "${pkgname}"
tar -zcf ../"${pkgname}.tar.gz" ./*

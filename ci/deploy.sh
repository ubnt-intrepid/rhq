#!/bin/bash

set -euo pipefail

pkgname="$1"
container_name=rust

docker exec -it "$container_name" cargo build --release

rm -rf ./"${pkgname}"
mkdir -p ./"${pkgname}"
cp ./target/release/rhq ./"${pkgname}"/

cd "${pkgname}"
tar -zcf ../"${pkgname}.tar.gz" ./*

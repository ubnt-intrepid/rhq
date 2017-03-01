#!/bin/bash

set -euo pipefail

pkgname=$1
target=$2

rm -rf ./"${pkgname}"
mkdir -p ./"${pkgname}"
cp ./target/"${target}"/release/rhq ./"${pkgname}"/

cd "${pkgname}"
tar -zcf ../"${pkgname}.tar.gz" ./*

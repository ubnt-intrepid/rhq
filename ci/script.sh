#!/bin/bash

# This script is copied from japaric/trust

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cross build --all --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --all --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi

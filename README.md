# `rhq` - Manages your local repositories

[![Crates.io](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Gitter](https://badges.gitter.im/ubnt-intrepid/rhq.svg)](https://gitter.im/ubnt-intrepid/rhq)

`rhq` is a CLI utility for management local repositories from decentrized version control systems (DVCSs).

## Installation

## Ubuntu / Debian

Requires [`cargo-deb`].

```shell-session
$ git clone https://github.com/ubnt-intrepid/rhq.git
$ cd rhq/
$ cargo deb --install
```

## From Source

```shell-session
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git
```

## Documentation (outdated)

- [README.md (old)](docs/README.md)
- [README.md (Japanese)](docs/README.ja.md)

## Alternatives
* motemen's [`ghq`](https://github.com/motemen/ghq)
* popomore's [`projj`](https://github.com/popomore/projj)

<!-- links -->

[`cargo-deb`]: https://github.com/mmstick/cargo-deb

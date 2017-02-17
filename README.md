# rhq

[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)
[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)

Manages your local repositories.

## Overview
`rhq` is a simple CLI tool which provides a way to organize and access local repositories,
cloned by Git and other VCSs.

```sh
$ rhq clone ubnt-intrepid/rhq
# Run `git clone https://github.com/ubnt-intrepid/rhq.git ~/.rhq/github.com/ubnt-intrepid/rhq`
```

```sh
$ rhq list
/home/username/.rhq/github.com/ubnt-intrepid/rhq
/home/username/.zplug/repos/zsh-users/zsh-autosuggestions
...
```

The development of this software is inspired by motemen's [`ghq`](https://github.com/motemen/ghq),
CLI tool for repository management written in Golang.

## Installation
The Rust toolchain is required to install `rhq`.
If you have already installed Rust toolchain:
```shell-session
$ cargo install rhq
```

Development version is available by using `--git` option as follows:
```shell-session
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git
```

## Usage

* `rhq clone [query] [-n | --dry-run] [--arg=<arg>]`  
  Clone remote reposities into the local directory.  
  `query` is a string to determine the URL of repository.
  Available formats of query are:
  * `(http|https|ssh|git)://github.com[:port]/username/repository.git`
  * `git@github.com:username/repository.git`
  * `github.com/username/repository`
  * `username/repository`

  If you want to pass supplemental arguments to `git` command, use `--arg="<arg>"`.

  `rhq` try to read standard input to get queries when `query` is omitted.
  This behaviour can use use like `ghq import`, as follows:
  ```sh
  cat list-of-queries.txt | rhq clone --arg="--recursive --depth 50"
  ```

* `rhq list`  
  List local repositories managed by rhq.

* `rhq completion [bash|zsh|fish|powershell]`  
  Generate completion script for your shell and dump to standard output.

If you want to see more information, use `rhq help`.

## Configureation
The location of configuration file is `~/.rhqconfig` or `~/.config/rhq/config`.

```toml
roots = [
  "~/.rhq",
  "~/.vim/plugged",
  "~/.zplug/repos",
  "~/.dotfiles"
]

clone_arg = "--depth 10"
```

## Interface of Visual Studio Code
* See [`vscode-rhq`](https://github.com/ubnt-intrepid/vscode-rhq).

## License
`rhq` is released under the MIT license. See [LICENSE](LICENSE) for details.

# `rhq` - Manages your local repositories

[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)
[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)

`rhq` is a command-line interface to manage local repositories, cloned by Git and other VCSs.

This software is inspired by motemen's [`ghq`](https://github.com/motemen/ghq),
CLI tool for repository management written in Golang.

## Overview
`rhq` provides a way to organize local repositories cloned by Git and other VCSs.

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

## Commands

###  `rhq clone [query] [-n | --dry-run] [--arg=<arg>]`

Clone remote reposities into the local directory.

* `query` : A string to determine the URL of repository.  
  Available query formats are:
  - `(http|https|ssh|git)://github.com[:port]/username/repository.git`
  - `git@github.com:username/repository.git`
  - `[github.com/]username/repository`

* `--arg=<arg>` : Supplemental arguments to pass `git` command.

### `rhq import [-n | --dry-run]`

Import remote repositories from standard input.

This behaviour can use like `ghq import`.
For example, to clone all of repositories owned by certain GitHub user:
```sh
curl -s "https://api.github.com/users/${user}/repos?max_pages=100" | jq -r '.[].name' | rhq import
```

### `rhq list`  
List local repositories managed by rhq.

### `rhq completion [bash|zsh|fish|powershell]`  
Generate completion script for your shell and dump to standard output.

## Configuration
The location of configuration file is `~/.rhqconfig` or `~/.config/rhq/config`.

```toml
# lookup directories to list local repositories.
# The first element is used by root directory to clone.
roots = [
  "~/.rhq",
  "~/.vim/plugged",
  "~/.zplug/repos",
  "~/.dotfiles"
]

# default argument to pass `git clone`
clone_arg = "--depth 10"
```

## Plugins for Text Editors
Extensions for Visual Studio Code is available. See [`vscode-rhq`](https://github.com/ubnt-intrepid/vscode-rhq) for details.

## License
`rhq` is released under the MIT license. See [LICENSE](LICENSE) for details.

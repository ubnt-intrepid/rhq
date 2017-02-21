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

### `rhq clone [query] [-n | --dry-run] [--arg=<arg>]`
Clone remote reposities into the local directory.

<!-- TODO: add `--protocol` option -->

* `[query]`  
  A string to determine the URL of remote repository. Available formats are:
  - URL: `(http|https|ssh|git)://github.com[:port]/username/repository.git`
  - SCP-like pattern: `git@github.com:username/repository.git`
  - relative path and hosts: `[github.com/]username/repository`

  When omitting, rhq get the list of queries from standard input.

* `--arg=<arg>`  
  Supplemental arguments to pass `git` command.

* `-n | --dry-run`  
  Show message string, instead of actually performing Git command.

### `rhq list`  
List local repositories managed by rhq.

### `rhq completion <shell> [out-file]`
Generate completion script for your shell.
If `out-file` is omitted, dump scirpt to standard output.

* `shell`: target shell `[bash|zsh|fish|powershell]`
* `out-file` : file path to write completion script

## Configuration
The behaviour of rhq can change by using configuration files.
The location of configuration file is `~/.rhqconfig` or `~/.config/rhq/config`.
Elements of configuration are as follows:

* `root` - string  
  The path of root directory to put in local repositories.
  The default value is `~/.rhq`.

* `supplements` - array of strings  
  Supplemental directories for lookup local repositories.

See [`.rhqconfig`](.rhqconfig) for details.

## Plugins for Text Editors
Extensions for Visual Studio Code is available. See [`vscode-rhq`](https://github.com/ubnt-intrepid/vscode-rhq) for details.

## License
`rhq` is released under the MIT license. See [LICENSE](LICENSE) for details.

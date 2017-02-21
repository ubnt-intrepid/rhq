# `rhq` - Manages your local repositories

[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)
[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![](http://vsmarketplacebadge.apphb.com/version-short/ubnt-intrepid.vscode-rhq.svg)](a)

`rhq` is a command-line tool to manage local repositories, cloned by Git and other VCSs.

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

### `rhq clone [<query>] [--arg=<arg>] [-n | --dry-run]`
Clone remote reposities into the local directory.

<!-- TODO: add `--protocol` option -->

* `<query>`  
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

### `rhq foreach [-n | --dry-run] <command> [<args>...]`
Execute commands into each local repositories.

* `<command>`  
  Command name
* `<args>...`  
  Supplemental arguments of command
* `-n | --dry-run`  
  Show message string, instead of actually performing Git command.

### `rhq completion <shell> [<out-file>]`
Generate completion script for your shell.
If `out-file` is omitted, dump scirpt to standard output.

* `<shell>`  
  Target shell name (value: `bash`, `zsh`, `fish` or `powershell`)
* `<out-file>`  
  Path to write completion script

## Configuration
The behaviour of rhq can change by using configuration files.
The location of configuration file is `~/.rhqconfig` or `~/.config/rhq/config`.

* `root` - string  
  The path of root directory to put in local repositories.
  The default value is `~/.rhq`.

* `supplements` - array of strings  
  Supplemental directories for lookup local repositories.

See [`.rhqconfig`](.rhqconfig) for details.

## Interface for Text Editors

### Vim
[`mattn/ctrlp-ghq`](https://github.com/mattn/ctrlp-ghq) is available.
If you are `vim-plug` user, try as follows:

```vim
Plug 'mattn/ctrlp-ghq'

let g:ctrlp_ghq_command = 'rhq'
let g:ctrlp_ghq_actions = [ { "label": "Open", "action": "Explore", "path": 0 } ]

noremap <Leader>g :<C-u>CtrlPGhq<CR>
```

### Visual Studio Code
Extensions for Visual Studio Code is available.
See [here](./vscode-rhq/README.md) for details.

## License
`rhq` is released under the MIT license. See [LICENSE](LICENSE) for details.

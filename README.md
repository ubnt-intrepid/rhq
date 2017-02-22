# `rhq` - Manages your local repositories

[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![](http://vsmarketplacebadge.apphb.com/version-short/ubnt-intrepid.vscode-rhq.svg)](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq)
[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)

`rhq` is a command-line repository management tool, written in Rust.

## Overview
`rhq` provides a way to organize local repositories cloned by Git and other VCSs.
You can use the command `rhq clone` as alternative of `git clone`,
to clone remote repositories under a specific root directory with intuitive directory structure.

```sh
$ rhq clone ubnt-intrepid/rhq
# Equivalent to `git clone https://github.com/ubnt-intrepid/rhq.git ~/.rhq/github.com/ubnt-intrepid/rhq`
```

```
~/.rhq/
  |- github.com/
  |  |- ubnt-intrepid/
  |  |  `- rhq/         <- clones with intuitive directory structure
  |  `- user2/
  |     `- repo3/
  `- gitlab.com/
     `- user3/
        `- repo4/
```

`rhq` also provides a way to list the location of managed local repositories.

```sh
$ rhq list
/home/username/.rhq/github.com/ubnt-intrepid/rhq
/home/username/.zplug/repos/zsh-users/zsh-autosuggestions
...
```

## Installation
The Rust toolchain is required. If you have already installed Rust toolchain:
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
See [here](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq) for details.

## License
`rhq` is released under the MIT license. See [LICENSE](LICENSE) for details.

## Similar projects
* motemen's [`ghq`](https://github.com/motemen/ghq)
* popomore's [`projj`](https://github.com/popomore/projj)

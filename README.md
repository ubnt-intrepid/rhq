# `rhq` - Manages your local repositories

[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![](http://vsmarketplacebadge.apphb.com/version-short/ubnt-intrepid.vscode-rhq.svg)](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq)
[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)
[![Gitter](https://badges.gitter.im/ubnt-intrepid/rhq.svg)](https://gitter.im/ubnt-intrepid/rhq?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

[Japanese](README.ja.md)

`rhq` is a repository management tool, written in Rust.
`rhq` provides a way to create/manage local repositories of decentrized version control systems (DVCSs).

Currently, only Git, Mercurial, Darcs and Pijul are supported.

## Installation
You can download prebuilt binaries from [GitHub releases page](https://github.com/ubnt-intrepid/rhq/releases).

If you have already installed Rust toolchain, You can build itself manually by using `cargo`, as follows:
```sh
# from crates.io
$ cargo install rhq
# from GitHub
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git
```

## Usage
See command line help for details.

### Clone Repository
To clone remote repository, use `rhq clone`.
Roughly speaking, this command can be used as commands like `git clone`.
For example, the command cloning this project is as follows:
```sh
$ rhq clone ubnt-intrepid/rhq
```
The first argument of `rhq clone` is a string which specify the remote repository.
Available patterns are:
* URL - `https://github.com/ubnt-intrepid/rhq.git`
* SCP - `git@github.com:ubnt-intrepid/rhq.git`
* Relative path - `ubnt-intrepid/rhq`  
  The host is fixed to `github.com`.

The second argument is target directory of cloned repository. If it is omitted, the location of cloned repository are determined from URL of remote repository, as follows:
```
~/.rhq/
  `- github.com/
     `- ubnt-intrepid/
        `- rhq/
```

### Add existed repositories into management
For adding existed repositories into management, the command `rhq add` is provided.
For example, your "dotfiles" repository can be add as follows:
```sh
$ rhq add ~/.dotfiles
```

By default, all arguments should be given as "absolute" path of added repository.
You can change this behavior by using an option `--import`, to find repositories from subdirectories of given paths.

For example, if you want to add all repositories cloned by Go toolchain:
```sh
$ rhq add --import $GOPATH/src
```

### Display and Manage Repositories
The list of managed repositories are saved to cache file.
If you want to list them, use `rhq list` as follows:
```sh
$ rhq list
```

To refresh information of managed repositories, use `rhq refresh`:
```sh
$ rhq refresh
```

## Configuration
The behaviour of rhq can change by using configuration files.
Configuration file is located at `~/.config/rhq/config.toml`.

The example of configuration file is as follows:

```toml
# The path of root directory to put in local repositories.
# The default value is `~/.rhq`.
root = "/path/to/repos"  

# Entry for lookup local repositories.
includes = [
  "~/go/src",
  "~/.vim/plugged",
  "~/.dotfiles"
]

# 
excludes = [
  "**/temp/*"
]
```

## Plugins

### Visual Studio Code
The owner of `rhq` also manages extension for Visual Studio Code.  
See [here](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq) for details.

## Similar projects
* motemen's [`ghq`](https://github.com/motemen/ghq)
* popomore's [`projj`](https://github.com/popomore/projj)

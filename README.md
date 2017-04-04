# `rhq` - Manages your local repositories

[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![](http://vsmarketplacebadge.apphb.com/version-short/ubnt-intrepid.vscode-rhq.svg)](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq)
[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)

`rhq` is a command-line repository management tool, written in Rust.

`rhq` provides a way to organize local repositories cloned by Git and other VCSs.  
You can use the command `rhq clone` as alternative of `git clone`,
to clone remote repositories under a specific root directory with intuitive directory structure.

## Example Usages
<!-- TODO: rewrite -->

### Create or Clone Repository

To clone Existed Remote Repository, use `rhq clone` as follows:
```sh
$ rhq clone ubnt-intrepid/rhq
# Equivalent to `git clone https://github.com/ubnt-intrepid/rhq.git ~/.rhq/github.com/ubnt-intrepid/rhq`
```

Cloned repositories are located under a specific root directory with intuitive directory structure:
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

### Display Managed Repositories
```sh
$ rhq list
/home/username/.rhq/github.com/ubnt-intrepid/rhq
/home/username/.zplug/repos/zsh-users/zsh-autosuggestions
...
```


## Installation
You can download precompiled artifacts from [GitHub releases page](https://github.com/ubnt-intrepid/rhq/releases).

If you have already installed Rust toolchain, you can build itself manually, with following command:
```shell-session
$ cargo install rhq  # from crates.io
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git  # from development repository
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
  "~/.dotfiles",
]
```

## Plugins

### For Vimmers
[`mattn/ctrlp-ghq`](https://github.com/mattn/ctrlp-ghq) is available.
If you are `vim-plug` user, try as follows:

```vim
Plug 'mattn/ctrlp-ghq'

let g:ctrlp_ghq_command = 'rhq'
let g:ctrlp_ghq_actions = [ { "label": "Open", "action": "Explore", "path": 0 } ]

noremap <Leader>g :<C-u>CtrlPGhq<CR>
```

### For Visual Studio Code Users
The developer is also managed an extension for Visual Studio Code.  
See [here](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq) for details.

## Similar projects
* motemen's [`ghq`](https://github.com/motemen/ghq)
* popomore's [`projj`](https://github.com/popomore/projj)

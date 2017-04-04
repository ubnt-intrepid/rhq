# `rhq` - リポジトリ管理を簡単に

[![](https://img.shields.io/crates/v/rhq.svg)](https://crates.io/crates/rhq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![](http://vsmarketplacebadge.apphb.com/version-short/ubnt-intrepid.vscode-rhq.svg)](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq)
[![Build Status](https://travis-ci.org/ubnt-intrepid/rhq.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/rhq)
[![Build status](https://ci.appveyor.com/api/projects/status/xc8i1sredjldkuy4?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/rhq)

## 概要
`rhq` は Rust で書かれたリポジトリ管理ツールです．  
本ツールを使用することで， Git や Mercurial などの分散バージョン管理システム (DVCS) で作成したローカルリポジトリをコマンドラインから簡単に管理することが出来ます．

本ツールは，以下のプロジェクトから着想を得て開発をしています．
* motemen's [`ghq`](https://github.com/motemen/ghq)  
  a
* popomore's [`projj`](https://github.com/popomore/projj)  
  a

### 特徴
* Supports for many DVCSs (Git, Mercurial, Darcs and Pijul)
* aa

## インストール
ビルド済みのバイナリは [GitHub のリリースページ](https://github.com/ubnt-intrepid/rhq/releases) からダウンロードが可能です．
現在は Windows, macOS, Linux および FreeBSD 向けのバイナリを用意しています．

Rust のツールチェインがインストールされている場合は `cargo` を用いたインストールが可能です．
```sh
# from crates.io
$ cargo install rhq
# from GitHub
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git
```

## 使い方
ここでは基本的な使用方法を説明します．
各コマンドのオプション一覧や詳細な仕様などはヘルプメッセージなどもご参照ください．

### リポジトリの作成・クローン
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

### 既存のリポジトリを管理対象に追加
`rhq add` is provided to add existed repositories into management.
For example, your "dotfiles" repository can be add like follows:
```sh
$ rhq add ~/.dotfiles
```

By default, you should give the arguments as 

If you, use option `--import`.

Repositories are detected and imported automatically.
```sh
# add all of repositories located under `~/go/src`
$ rhq add --import --verbose ~/go/src
Added /home/user1/go/src/github.com/ubnt-intrepid/go-git-prompt
...
```

### 管理下のリポジトリの表示・更新
The list of managed repositories are saved to cache file.
If you want to list them, use `rhq list` as follows:
```sh
$ rhq list
/home/username/.rhq/github.com/ubnt-intrepid/rhq
/home/username/.zplug/repos/zsh-users/zsh-autosuggestions
...
```

## 設定
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

## プラグイン

### For Visual Studio Code Users
The developer is also managed an extension for Visual Studio Code.  
See [here](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq) for details.

### For Vimmers
[`mattn/ctrlp-ghq`](https://github.com/mattn/ctrlp-ghq) is available.
If you are `vim-plug` user, try as follows:

```vim
Plug 'mattn/ctrlp-ghq'

let g:ctrlp_ghq_command = 'rhq'
let g:ctrlp_ghq_actions = [ { "label": "Open", "action": "Explore", "path": 0 } ]

noremap <Leader>g :<C-u>CtrlPGhq<CR>
```

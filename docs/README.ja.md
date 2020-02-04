# `rhq` - リポジトリ管理を簡単に

[English](README.md)

`rhq` は Rust で書かれたリポジトリ管理ツールです．
本ツールを使用することで， Git や Mercurial に代表される分散バージョン管理システム (DVCS) によるリポジトリの作成・管理を簡素化することが出来ます．

## インストール
ビルド済みのバイナリは [GitHub のリリースページ](https://github.com/ubnt-intrepid/rhq/releases) からダウンロードが可能です．
適当なディレクトリに展開し，解凍したディレクトリ下の `bin/` にパスを通してください．
現在は Windows, macOS, Linux および FreeBSD 向けのバイナリを用意しています．

すでに Rust のツールチェインがインストールされている場合は `cargo` を用いてインストールすることも出来ます．
```sh
# from crates.io
$ cargo install rhq
# from GitHub
$ cargo install --git https://github.com/ubnt-intrepid/rhq.git
```

## 使い方
ここでは，よく使用されるコマンドの基本的な使用方法を説明します．
省略されているコマンドや各コマンドのオプションの一覧はヘルプメッセージを参照してください．

### リポジトリのクローン
リモートリポジトリのクローンには `rhq clone` を使用します．
このコマンドは基本的には `git clone` など既存の VCS がクローン用に用意したコマンドと同様に用いることが出来ます．
例えば，このプロジェクトのリポジトリをクローンするには次のように実行します．
```sh
$ rhq clone ubnt-intrepid/rhq [/path/to/rhq]
```

`rhq clone` の第一引数にはリモートリポジトリを指定する文字列，第二引数にはクローン先のディレクトリを指定します．
第一引数に渡すことのできる文字列のパターンは以下の通りです．
* URL - `https://github.com/ubnt-intrepid/rhq.git`
* SCP - `git@github.com:ubnt-intrepid/rhq.git`
* 相対パス - `ubnt-intrepid/rhq`

現状，相対パスを指定したときに補完されるホスト名は `github.com` に固定されているので注意してください．

第二引数は省略可能であり，省略した場合はリモートリポジトリの URL をもとにクローン先のディレクトリが決定されます．
例えば，先ほどクローンしたリポジトリは次のようなディレクトリ構造で保存されます．
```
~/.rhq/
  `- github.com/
     `- ubnt-intrepid/
        `- rhq/
```

### リポジトリを管理対象に追加する
クローン・作成済みのリポジトリを管理下に含めるには `rhq add` を使用します．
例えば，いわゆる "dotfiles" 用のリポジトリを管理したいときは次のようにします．
```sh
$ rhq add ~/.dotfiles
```

`rhq add` の引数には追加したいリポジトリのパスを指定します．
デフォルトでは，各引数の値は”厳密に”リポジトリのパスを指している必要があります（それ以外のパスを指定した場合は無視されます）．
この挙動は `--import` により変更することができ，このオプションが指定されている場合は各パス内にあるリポジトリを検索し逐次管理対象に追加します．

例えば，Go 言語のワークスペース内にあるリポジトリをすべて管理対象に含めたい場合は次のようにします．
```sh
$ rhq add --import --verbose $GOPATH/src
```


### 管理下のリポジトリの表示・更新
`rhq` は管理しているリポジトリの一覧をキャッシュファイルとして保持しています．
この内容を表示するためには `rhq list` コマンドを使用します．
```sh
$ rhq list
```

キャッシュの内容は `rhq refresh` を用いて更新します．
```sh
$ rhq refresh
```

## 設定
設定ファイルは `~/.config/rhq/config.toml` に配置します．

例:
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

### Visual Studio Code
Visual Studio Code 向けの拡張機能を作りました．
詳細は[こちら](https://marketplace.visualstudio.com/items?itemName=ubnt-intrepid.vscode-rhq)を参照してください．


## ライセンス
MIT

## 類似プロジェクト
* motemen 氏の [ghq](https://github.com/motemen/ghq)
* popomore 氏の [projj](https://github.com/popomore/projj)
# 概要の説明とドキュメント

[docs/src](docs/src/SUMMARY.md)ディレクトリにメインのマークダウンドキュメントがあり、[docs/book](docs/book)ディレクトリにそれをmdbookを使用してHTMLに変換したものがある。

## How to read

リポジトリをcloneして`docs/book/index.html`をブラウザで開く。

```sh
open docs/book/index.html
```

または[docs/src/SUMMARY.md](docs/src/SUMMARY.md)からマークダウンを読む。

## 他のドキュメント

- [2021-02-19のインターン成果発表での資料](2021-02-19-summary.md)

## Development

mdbookのインストール:

```sh
# See https://github.com/rust-lang/mdBook#installation for more.
cargo install mdbook
```

docsディレクトリに移動。

```sh
cd docs
```

`mdbook build`で`docs/book`にHTMLが生成される。

```sh
mdbook build
```

ドキュメントを変更しながらその結果を確認したい場合は`mdbook serve`が便利。

```sh
mdbook serve
```

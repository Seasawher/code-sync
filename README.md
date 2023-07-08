# Code Sync

コードの，同時に変更しなければならないファイル群をチェックするツールです．

## 参考にしたもの

* [Command Line Applications in Rust](https://rust-cli.github.io/book/) Rust でCLIツールを作ることに関する本.
* [fd-find](https://github.com/sharkdp/fd) Rustによる高速な `find` の再実装．ライブラリとしてCLIツールの内部で使えればよかったが，それはできない．
* [ignore](https://crates.io/crates/ignore) fd-findの内部で使用されているファイル検索クレート．
* [clap](https://crates.io/crates/clap) コマンドラインツールを作るためのクレート．
* [tempdir](https://crates.io/crates/tempdir) 一時ファイルを作成することができる，テストを楽にしてくれそうなクレート.

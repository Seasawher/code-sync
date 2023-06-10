# mono-search

ファイル検索CLIツール

## 要件

### 機能

* あるディレクトリの中の，特定の条件を満たすファイルを検索することができる
  * 検索にヒットしたファイルがどのファイルか，パスも含めて出す
  * どのファイルの何行目がヒットしたかも示す
  * ヒットしたファイルの数をサマリとして示す

* 検索対象のファイルを設定する条件は，以下から自由に複数選択可能
  * 与えられた正規表現にマッチする行を含む
  * 与えられた文字列にファイル名が一致する
  * ファイル名として，正規表現も指定できる

* 探す対象となるディレクトリは複数指定することができる
  * 探す対象となるディレクトリの階層は問わない
  * 探す対象となるディレクトリは，CLIツールがインストールされたカレントディレクトリおよびその配下に限り，親ディレクトリは探索できない

### UI

* 探す対象となるディレクトリにいくつファイルがあるか数えて，検索作業の進捗をリアルタイムに出す

### そのほか

* 5万ファイル程度のディレクトリを検索しても，3秒以内に処理が終わる
* 単独で使うことができて，依存ツールのインストールが不要
* 完全にローカルに動作し，ネットにアクセスしない
* ファイルの読み込みのみを行い，変更・削除をしない
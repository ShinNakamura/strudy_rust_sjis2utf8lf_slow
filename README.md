## 注意

Rust 学習中の練習用リポジトリ

学習のために作成してるコードになりますので、クローンして使用する際は自己責任でお願い致します。

このコードは遅い

同じことをするなら `https://github.com/ShinNakamura/study_rust_sjis2utf8lf` のほうが高速

とはいえ、Rust 学習では学びが多かったのでリポジトリに残しておく

## 説明

標準入力から、もしくは、コマンドライン引数からファイルパスを1つだけ読み込んで
そのファイルが `Shift_JIS` である前提で(改行コードは切り捨てられる)
utf8 に変換し、改行コード LF (というか、プラットフォームの改行コード)で標準出力へ送る

## 参考

https://stackoverflow.com/questions/64040851/how-can-i-read-a-non-utf8-file-line-by-line-in-rust

@echo off
rem task-reporter 実行用
rem 改行コードは「CRLF」にしないと動かない

rem Rustプロジェクトのルートディレクトリに移動する
cd /d %~dp0/..

rem 画面サイズ変更(表示位置はコマンドプロンプトのプロパティを調整する)
mode con: cols=185 lines=1000

rem cargo経由で実行する
cargo run -p task-reporter -- %* | bunyan

pause
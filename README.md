# task-reporter

タスク管理ツールからタスク毎の所要時間を取得し、集計して表示するCLIツール

# 要件

- タスク管理アプリには「[ClickUp](https://app.clickup.com/)」を利用する
- 対象日、あるいは対象期間のタスク情報を取得する
- 1日の開始は午前5時、終了は午後4時59分とする
- 出力パターンは複数存在する
    - 1日単位（対象期間は1日のみ）、タスク毎
    - 1日単位（対象期間は1日のみ）、チャージ毎
    - 1日単位（対象期間は複数日に渡る）、タスク毎
    - 1日単位（対象期間は複数日に渡る）、チャージ毎
    - 対象期間単位（対象期間は複数日に渡る）、タスク毎
    - 対象期間単位（対象期間は複数日に渡る）、チャージ毎
- 出力する情報は下記の通り
    - 対象日あるいは対象期間
    - ステータス
    - タスク名
    - チャージコード
    - 所要時間（hh:mi:ss表記）
- 出力形式は、対象日あるいは対象期間は別枠とし、他はTSV形式とする
- 出力TSVの末尾には所要時間の総合計を表示する
- オプション指定によって、出力結果をクリップボードにコピーできる
- オプション指定によって、対象期間内に最終更新のあったタスク情報（所要時間を持たない）も表示する


# 利用前の準備

- dotenv/.env.sample の中身を適宜書き換え、dotenv/.env にリネームする
- bat/task-reporter.bat に 環境変数PATH を通す
- 別途 [bunyan-rs](https://github.com/LukeMathWalker/bunyan) をインストールしておく


# Usage

このリポジトリをクローンして利用する場合を想定しているため、
下記 `task-reporter.exe` は `task-reporter.bat` に読み替えること。

```ps1
Task Reporter 0.1.0
zumi
This is a CLI tool that aggregates and displays work hours collected from a task management app by
charge or task.

USAGE:
    task-reporter.exe [OPTIONS] [ARGS]

ARGS:
    <START_DATE>    始端日をYYYY/MM/DD形式で指定する
    <END_DATE>      終端日をYYYY/MM/DD形式で指定する

OPTIONS:
    -a, --all              デフォルトでは対象期間内のタイムエントリーのみを表示する。このフラグを指定すると、最終更新日時が対象期間内であるタスク情報も表示する
    -c, --by-charge        デフォルトではタスク単位で集計する。このフラグを指定すると、チャージコード単位で集計する
    -d, --by-daily         デフォルトでは対象期間単位で集計する。このフラグを指定すると、日単位で集計する
    -h, --help             Print help information
    -s, --set-clipboard    このフラグを指定すると、結果をクリップボードにセットする
    -V, --version          Print version information
```



# PowerShell で利用する場合

## 文字化け対策

PowerShell内部の文字エンコーディングの問題で、
bunyanによるログ出力が文字化けする可能性がある。
それを解消するため、プロファイルに下記を追記する。

```Microsoft.PowerShell_profile.ps1
# 文字列をパイプする時の文字コードをUTF-8に設定する。
[Console]::OutputEncoding = [System.Text.Encoding]::GetEncoding('utf-8')
$OutputEncoding = [console]::OutputEncoding
```

### ログ調査用の設定

ログを整形して出力する場合、下記コマンドを使えば良い。
※ [bat](https://github.com/sharkdp/bat) や [ripgrep](https://github.com/BurntSushi/ripgrep) をインストールしていることを前提としている。


```ps1
bat --style plain <file> | rg <pattern> | bunyan --level <level> 
```

しかしながら、長いコマンドを打つのは面倒なので、
下記のように関数を作っておくと楽ができる。

```Microsoft.PowerShell_profile.ps1
# bunyan でログをパイプして読むエイリアス
# 何故か1行目が整形されないので、echoで無理やり追加している
function bunlog() {
    Param(
        [parameter(mandatory)][String]$path,
        [String]$pattern = ".*",
        [ValidateSet("trace" , "debug", "info", "warn", "error", "fatal")]$level = "trace"
    )
    if ($pattern -eq ".*") {
        echo $(echo "[bunyan formatted text]" && bat --style plain $path) `
            | bunyan --level $level
    } else {
        echo $(echo "[bunyan formatted text (pattern: ${pattern} )]" && bat --style plain $path) `
            | rg -e $pattern `
            | bunyan --level $level
    }
}
```

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


# 利用前の準備

- dotenv/.env.sample の中身を適宜書き換え、dotenv/.env にリネームする
- bat/task-reporter.bat に 環境変数PATH を通す
- 別途 [bunyan-rs](https://github.com/LukeMathWalker/bunyan) をインストールしておく


# Usage

結果は標準出力に表示される。
また、クリップボードにもコピーされる（Windowsのみ対応）

```ps1
# 1日単位（対象期間は1日のみ）、タスク毎
task-repoter.bat <yyyy/mm/dd>

# 1日単位（対象期間は1日のみ）、チャージ毎
task-repoter.bat <yyyy/mm/dd> --by-charge

# 1日単位（対象期間は複数日に渡る）、タスク毎
task-repoter.bat <yyyy/mm/dd> <yyyy/mm/dd> --by-daily

# 1日単位（対象期間は複数日に渡る）、チャージ毎
task-repoter.bat <yyyy/mm/dd> <yyyy/mm/dd> --by-daily --by-charge

# 対象期間単位（対象期間は複数日に渡る）、タスク毎
task-repoter.bat <yyyy/mm/dd> <yyyy/mm/dd> 

# 対象期間単位（対象期間は複数日に渡る）、チャージ毎
task-repoter.bat <yyyy/mm/dd> <yyyy/mm/dd> --by-charge
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

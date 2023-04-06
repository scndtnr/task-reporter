# task-reporter
タスク管理ツールからタスク毎の経過時間を取得し、集計して表示するCLIツール

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
    - タスク名
    - チャージコード
    - 経過時間（hh:mi:ss表記）
- 出力形式は、対象日あるいは対象期間は別枠とし、他3つはTSV形式とする


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ByTaskAndTotalDto {
    // 最終更新日時
    updated_at: String,
    // チャージコード
    charge_name: String,
    // タスクのステータス
    task_status: String,
    // タスク名
    task_name: String,
    // 経過時間（hh:mi:ss表記）
    duration: String,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ByTaskAndTotalDtos {
    // 対象日あるいは対象期間
    start_date: String,
    end_date: Option<String>,
    // 対象期間全体・タスク単位の集計結果一覧
    dtos: Vec<ByTaskAndTotalDto>,
}

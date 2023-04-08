use clap::Parser;

/// コマンドライン引数のパース用構造体
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "Task Reporter",
    version = "0.1.0",
    author = "zumi",
    about = "This is a CLI tool that aggregates and displays work hours collected from a task management app by charge or task."
)]
#[clap(propagate_version = true)]
pub struct Opts {
    #[clap(help = "始端日をYYYY/MM/DD形式で指定する")]
    start_date: Option<String>,
    #[clap(help = "終端日をYYYY/MM/DD形式で指定する")]
    end_date: Option<String>,
    #[clap(short = 'c', long, help = "チャージコード単位で集計するboolフラグ")]
    by_charge: bool,
    #[clap(short = 'd', long, help = "日単位で集計するboolフラグ")]
    by_daily: bool,
}

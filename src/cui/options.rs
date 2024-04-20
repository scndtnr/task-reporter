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
pub(crate) struct Opts {
    #[clap(help = "始端日をYYYY/MM/DD形式で指定する")]
    start_date: Option<String>,
    #[clap(help = "終端日をYYYY/MM/DD形式で指定する")]
    end_date: Option<String>,
    #[clap(
        short = 'c',
        long,
        help = "デフォルトではタスク単位で集計する。このフラグを指定すると、チャージコード単位で集計する"
    )]
    by_charge: bool,
    #[clap(
        short = 'd',
        long,
        help = "デフォルトでは対象期間単位で集計する。このフラグを指定すると、日単位で集計する"
    )]
    by_daily: bool,
    #[clap(
        short = 'a',
        long,
        help = "デフォルトでは対象期間内のタイムエントリーのみを表示する。このフラグを指定すると、最終更新日時が対象期間内であるタスク情報も表示する"
    )]
    all: bool,
    #[clap(
        short = 's',
        long,
        help = "このフラグを指定すると、結果をクリップボードにセットする"
    )]
    set_clipboard: bool,
}

impl Opts {
    pub(crate) fn start_date(&self) -> &Option<String> {
        &self.start_date
    }
    pub(crate) fn end_date(&self) -> &Option<String> {
        &self.end_date
    }
    pub(crate) fn by_charge(&self) -> bool {
        self.by_charge
    }
    pub(crate) fn by_daily(&self) -> bool {
        self.by_daily
    }
    pub(crate) fn all(&self) -> bool {
        self.all
    }
    pub(crate) fn set_clipboard(&self) -> bool {
        self.set_clipboard
    }
}

pub(super) enum AggregateCondition {
    TaskAndTotalPeriod,
    ChargeAndTotalPeriod,
    TaskAndDaily,
    ChargeAndDaily,
}

impl From<Opts> for AggregateCondition {
    fn from(opts: Opts) -> Self {
        if !opts.by_charge() && !opts.by_daily() {
            AggregateCondition::TaskAndTotalPeriod
        } else if opts.by_charge() && !opts.by_daily() {
            AggregateCondition::ChargeAndTotalPeriod
        } else if !opts.by_charge() && opts.by_daily() {
            AggregateCondition::TaskAndDaily
        } else if opts.by_charge() && opts.by_daily() {
            AggregateCondition::ChargeAndDaily
        } else {
            unreachable!()
        }
    }
}

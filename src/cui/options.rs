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
    #[clap(short = 'c', long, help = "チャージコード単位で集計するboolフラグ")]
    by_charge: bool,
    #[clap(short = 'd', long, help = "日単位で集計するboolフラグ")]
    by_daily: bool,
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

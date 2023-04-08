use crate::cui::Opts;

#[derive(Debug, Clone)]
pub(crate) struct RequestDto {
    start_date: Option<String>,
    end_date: Option<String>,
    by_charge: bool,
    by_daily: bool,
}

impl RequestDto {
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

impl From<Opts> for RequestDto {
    fn from(opts: Opts) -> Self {
        Self {
            start_date: opts.start_date().clone(),
            end_date: opts.end_date().clone(),
            by_charge: opts.by_charge(),
            by_daily: opts.by_daily(),
        }
    }
}

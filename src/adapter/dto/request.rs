use crate::cui::Opts;

#[derive(Debug, Clone)]
pub(crate) struct RequestDto {
    start_date: Option<String>,
    end_date: Option<String>,
    all: bool,
}

impl RequestDto {
    pub(crate) fn start_date(&self) -> &Option<String> {
        &self.start_date
    }
    pub(crate) fn end_date(&self) -> &Option<String> {
        &self.end_date
    }
    pub(crate) fn all(&self) -> bool {
        self.all
    }
}

impl From<Opts> for RequestDto {
    fn from(opts: Opts) -> Self {
        Self {
            start_date: opts.start_date().clone(),
            end_date: opts.end_date().clone(),
            all: opts.all(),
        }
    }
}

use clap::Parser;

mod options;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    // controller: Controller<UsecaseImpls>,
    opts: options::Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        Self {
            opts: options::Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        tracing::debug!("cli args: {:#?}", self.opts);
    }
}

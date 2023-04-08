mod adapter;
mod cui;
mod domain;
mod env;
mod logging;

pub async fn init() {
    env::set_dotenv("task-reporter");
    logging::init_logging_with_bunyan();
    tracing::debug!("Task Reporter Process Start");
    let app = cui::Cui::new().await;
    app.process().await;
}

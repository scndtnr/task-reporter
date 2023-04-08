mod cui;
mod env;
mod logging;

pub async fn init() {
    env::set_dotenv("task-reporter");
    logging::init_logging_with_bunyan();
    tracing::debug!("Hello");
    // let app = cui::Cui::new().await;
    // app.process_cmd().await;
}

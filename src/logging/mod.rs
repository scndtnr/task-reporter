mod filter;
mod filtered_layer;
mod layer;
mod writer;

use crate::env::get_env_var;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(super) fn init_logging_with_bunyan() {
    let service_name = get_env_var("SERVICE_NAME").unwrap();
    // ログ設定にフィルタ・フォーマットを登録し適用する
    tracing_subscriber::registry()
        // --- bunyan formatting layer ---
        .with(JsonStorageLayer)
        .with(filtered_layer::bunyan_stdio_of_app(&service_name))
        .with(filtered_layer::bunyan_file_of_app(&service_name))
        .init();
}

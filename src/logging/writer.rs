use crate::env::get_env_var;

use chrono::Utc;
use std::path::PathBuf;
use tracing_appender::rolling::RollingFileAppender;

// 標準出力用のWriter
// std::io::stdout を使えば良い

/// ファイル出力用のWriter
pub(super) fn rolling_file() -> RollingFileAppender {
    // ログディレクトリパスとファイル名を生成する
    let dirpath = log_dirpath();
    let filename = get_env_var("LOG_FILENAME").unwrap();

    // ログファイルのWriterを生成する
    tracing_appender::rolling::hourly(dirpath, filename)
}

fn log_dirpath() -> PathBuf {
    // 実行時のディレクトリを取得する
    let current_dir = match std::env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => panic!("Fail to get current directory\n{:#?}", e),
    };

    // 環境変数からログディレクトリへのPATH要素を取得する
    let log_dir = get_env_var("LOG_DIR").unwrap();
    let today = Utc::now().format("%Y-%m-%d").to_string();

    // ディレクトリパスを作成する
    current_dir.join(log_dir).join(today)
}

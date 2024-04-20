use tracing::Subscriber;
use tracing_appender::rolling::RollingFileAppender;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    filter::{Filtered, Targets},
    registry::LookupSpan,
    Layer,
};

// type: filtered by targets

type BunyanStdioLayerFilterdByTargets<S> =
    Filtered<BunyanFormattingLayer<fn() -> std::io::Stdout>, Targets, S>;
type BunyanRollingFileLayerFilterdByTargets<S> =
    Filtered<BunyanFormattingLayer<RollingFileAppender>, Targets, S>;

// app log

/// (app log) bunyan形式で標準出力に書き込むフォーマッタ
pub(crate) fn bunyan_stdio_of_app<S>(name: &str) -> BunyanStdioLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let stdio_filter = super::filter::app_only(true);

    super::layer::bunyan_stdio_format(name).with_filter(stdio_filter)
}

/// (app log) bunyan形式でファイルに書き込むフォーマッタ
pub(crate) fn bunyan_file_of_app<S>(name: &str) -> BunyanRollingFileLayerFilterdByTargets<S>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let file_filter = super::filter::app_only(false);
    let make_writer = super::writer::rolling_file();

    super::layer::bunyan_file_format(name, make_writer).with_filter(file_filter)
}

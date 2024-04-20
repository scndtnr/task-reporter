use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::fmt::MakeWriter;

/// bunyan形式で標準出力に書き込むフォーマッタ
pub(crate) fn bunyan_stdio_format(
    name: impl Into<String>,
) -> BunyanFormattingLayer<fn() -> std::io::Stdout> {
    BunyanFormattingLayer::new(name.into(), std::io::stdout)
}

/// bunyan形式でファイルに書き込むフォーマッタ
pub(crate) fn bunyan_file_format<W>(
    name: impl Into<String>,
    make_writer: W,
) -> BunyanFormattingLayer<W>
where
    W: for<'a> MakeWriter<'a> + 'static,
{
    BunyanFormattingLayer::new(name.into(), make_writer)
}

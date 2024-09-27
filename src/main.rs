use opp::ast::AnyRes;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, format::FmtSpan, time::ChronoLocal},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[cfg(debug_assertions)]
const fn get_level_filter() -> LevelFilter {
    LevelFilter::DEBUG
}

#[cfg(not(debug_assertions))]
const fn get_level_filter() -> LevelFilter {
    LevelFilter::INFO
}

fn init_tracing() {
    // See: https://github.com/tokio-rs/tracing/issues/3068
    #[cfg(windows)]
    nu_ansi_term::enable_ansi_support().ok();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_timer(ChronoLocal::new("%m-%d %H:%M:%S".into()))
                .with_span_events(FmtSpan::FULL)
                .with_thread_names(true)
                .with_filter(get_level_filter()),
        )
        .init();
}

#[tokio::main]
async fn main() -> AnyRes {
    init_tracing();
    tracing::info!("Hello, world!");
    Ok(())
}

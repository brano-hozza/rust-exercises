use std::{fs::File, sync::Arc};

// A layer that logs events to stdout using the human-readable "pretty"
// format.
use tracing_subscriber::{filter, prelude::*};
fn main() {
    use tracing::{event, span, Level};

    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    let debug_file = File::create("debug.log").expect("Unable to create debug.log");
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(debug_file));

    tracing_subscriber::registry()
        .with(stdout_log.with_filter(filter::LevelFilter::INFO))
        .with(debug_log.with_filter(filter::LevelFilter::DEBUG))
        .init();

    // records an event outside of any span context:
    event!(Level::INFO, "something happened");
    event!(Level::DEBUG, "only for debug");

    let span = span!(Level::INFO, "my_span");
    let _guard = span.entered();

    // records an event within "my_span".
    event!(Level::INFO, "something happened inside my_span");

    _guard.exit();
    // records an event outside "my_span".
    event!(Level::INFO, "something happened outside my_span");
}

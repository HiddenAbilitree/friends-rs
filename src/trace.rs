#[cfg(feature = "tracing")]
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};

use tracing_panic::panic_hook;
use tracing_subscriber::prelude::*;

// hold the tracing_chrome guard in the outer scope
pub struct TraceGuard {
    #[cfg(feature = "tracing")]
    _tracing_chrome_guard: FlushGuard,
}

// load all the tracing settings including whether
// to enable tracing_chrome via the tracing feature,
// see README.md
pub fn trace_init() -> TraceGuard {
    #[cfg(feature = "tracing")]
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();

    #[cfg(feature = "tracing")]
    tracing_subscriber::registry()
        .with(chrome_layer)
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    #[cfg(not(feature = "tracing"))]
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    std::panic::set_hook(Box::new(panic_hook));

    #[cfg(feature = "tracing")]
    return TraceGuard {
        _tracing_chrome_guard: _guard,
    };

    #[cfg(not(feature = "tracing"))]
    TraceGuard {}
}

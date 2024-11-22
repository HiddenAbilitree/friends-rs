mod data;
mod trace;

use anyhow::{Context, Result};
use data::initial_data_load;
use tracing::{span, Level};
use xdg::BaseDirectories;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = trace::trace_init();
    let load_span = span!(Level::INFO, "main");
    let _load_enter = load_span.enter();

    let base_dirs = BaseDirectories::new().context("Failed to find BaseDirectories")?;
    initial_data_load(&base_dirs).await?;

    Ok(())
}

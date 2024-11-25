use core::str;
use std::path::PathBuf;

use anyhow::{Context, Result};
use comrak::{parse_document, Arena, Options};
use tokio::fs;
use tracing::debug;
use xdg::BaseDirectories;

const APP: &str = "friends-rs";

#[tracing::instrument(level = "info")]
pub async fn initial_data_load(base_dirs: &BaseDirectories) -> Result<()> {
    let friends_file = base_dirs.get_data_file(format!("{}/friends.md", APP));

    debug!(?friends_file, "resolved data path");

    let friends_file_exists = friends_file
        .try_exists()
        .context("Failed to check for the existence of friends.md")?;

    debug!(?friends_file_exists);

    let friends_file_exists = friends_file
        .try_exists()
        .context("Failed to check for the existence of friends.md")?;

    if friends_file_exists {
        let friends_contents = fs::read(&friends_file)
            .await
            .context("Failed to read friends.md")?;

        debug!("building index from scratch");

        rebuild_index(Some(friends_contents), &friends_file).await?;
        return Ok(());
    }

    debug!("building friends.md from scratch");
    rebuild_index(None, &friends_file).await?;

    Ok(())
}

#[tracing::instrument(level = "info")]
async fn rebuild_index(contents: Option<Vec<u8>>, friends_file: &PathBuf) -> Result<()> {
    debug!("rebuilding index");

    match contents {
        Some(content) => {
            let arena = Arena::new();
            let root = parse_document(&arena, str::from_utf8(&content)?, &Options::default());

            todo!("build index from friends.md")
        }
        None => {
            // todo(kunet): markdown generation from AST

            fs::write(
                friends_file,
                "### Activities:\n\n### Notes:\n\n### Friends:\n\n### Locations:\n\n",
            )
            .await
            .context("Failed to write default friends.md")?;
        }
    };

    Ok(())
}

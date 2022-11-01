mod archive;
mod cmd;
mod extents;
mod operation;
mod partition;
mod payload;

use crate::cmd::extract::extract;
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Extract(ExtractCmd),
}

#[derive(Parser)]
pub struct ExtractCmd {
    ota_path: PathBuf,
    #[clap(short, long)]
    out_path: Option<PathBuf>,
    #[clap(long)]
    no_verify: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Args::parse();
    match cli.command {
        Command::Extract(cmd) => extract(cmd).await?,
    }

    Ok(())
}

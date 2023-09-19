#![feature(read_buf)]

mod archive;
mod bspatch;
mod cmd;
mod extents;
mod hashtree;
mod operation;
mod partition;
mod payload;
mod puffpatch;

use crate::cmd::apply::apply;
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
    Apply(ApplyCmd),
}

#[derive(Parser)]
pub struct ExtractCmd {
    ota: PathBuf,
    #[clap(short, long)]
    out: Option<PathBuf>,
    #[clap(long)]
    no_verify: bool,
}

#[derive(Parser)]
pub struct ApplyCmd {
    /// The new update (diff) OTA to apply
    ota: PathBuf,
    /// The path to the extracted previous OTA that this update OTA applies to
    #[clap(short, long)]
    previous: PathBuf,
    #[clap(short, long)]
    out: Option<PathBuf>,
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
        Command::Apply(cmd) => apply(cmd).await?,
    }

    Ok(())
}

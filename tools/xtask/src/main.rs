mod config;
mod metadata;
mod tasks;

use crate::metadata::load_cargo_metadata;
use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use smol::fs;
use std::{fs::File, path::PathBuf};
use tasks::TaskRunner;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    FmtSubscriber, fmt, fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate tracing;

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
    #[clap(short, long, value_enum)]
    info_level: Option<InfoLevel>,
    #[clap(short, long)]
    file_path: Option<PathBuf>,
    #[clap(long, action)]
    /// Terminate the program once a rule violation is found
    fail_fast: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum InfoLevel {
    TRACE,
    INFO,
    WARN,
}

#[derive(Subcommand)]
enum CliCommand {
    #[command(name = "license")]
    License(tasks::license::LicenseCommandArgs),

    #[command(name = "audit")]
    Audit(tasks::audit::AuditCommandArgs),

    #[command(name = "cargo-features")]
    CargoFeatures(tasks::cargo_features::CargoFeaturesCommandArgs),
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let fail_fast = args.fail_fast;
    let info_level = match args.info_level.unwrap_or(InfoLevel::INFO) {
        InfoLevel::WARN => Level::WARN,
        InfoLevel::INFO => Level::INFO,
        InfoLevel::TRACE => Level::TRACE,
    };

    // TODO: Make output to file work
    // Right now the info_level setting works for console output
    if let Some(file_path) = args.file_path {
        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }
        let _ = File::create(&file_path);
        let logfile = RollingFileAppender::new(Rotation::NEVER, &file_path, "xtask.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(logfile);
        let file_layer = fmt::layer().with_writer(non_blocking.with_max_level(info_level));

        tracing_subscriber::registry().with(file_layer).init();
    } else {
        let subscriber = FmtSubscriber::builder().with_max_level(info_level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .context("setting default subscriber failed")?;
    }

    let metadata = load_cargo_metadata()?;
    let mut runner = TaskRunner::new();

    match args.command {
        CliCommand::License(args) => {
            runner.spawn_job(tasks::license::run_license(args, metadata));
            runner.run().await
        }
        CliCommand::Audit(args) => {
            runner.spawn_job(tasks::audit::check_dependencies_job(
                args, metadata, fail_fast,
            ));
            runner.run().await
        }
        CliCommand::CargoFeatures(args) => {
            runner.spawn_job(tasks::cargo_features::run_cargo_features(args, metadata));
            runner.run().await
        }
    }
}

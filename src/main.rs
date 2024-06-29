use clap::{Args, Parser, Subcommand, ValueEnum};
use map::GunfightMap;
use std::path::PathBuf;
use strum_macros::Display;
use tracing_log::AsTrace;

use crate::otel::setup_otel;
use crate::run::run;

const DAY_FMT: &str = "%m-%d-%Y";

pub mod error;
pub mod graph;
pub mod map;
pub mod menus;
pub mod otel;
pub mod run;
pub mod stats;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Commands,

    #[clap(flatten)]
    pub args: SharedArgs,
}

#[derive(Subcommand, Debug, Display)]
pub enum Commands {
    Graph,
    Prompt,
}

#[derive(Args, Debug)]
pub struct SharedArgs {
    #[arg(short, long)]
    stats_path: PathBuf,

    #[arg(short, long)]
    cod_version: CodVersion,
}

#[derive(Debug, ValueEnum, Display, Clone)]
pub enum CodVersion {
    MW,
    MW3,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let guard = setup_otel(cli.verbose.log_level_filter().as_trace());

    if run(cli).is_err() {
        drop(guard);
        std::process::exit(1);
    }
}

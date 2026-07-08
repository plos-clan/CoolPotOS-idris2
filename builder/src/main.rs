use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod cargo;
mod commands;
mod image;

use crate::commands::*;

/// CoolPotOS kernel builder, tester and runner
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Build the kernel.
    Build(BuildArgs),
    /// Run the kernel in QEMU.
    Run(RunArgs),
    /// Run cargo clippy for the kernel.
    Clippy,
    /// Run tests.
    Test,
}

#[derive(Args)]
struct BuildArgs {
    /// Build with release profile
    #[clap(long)]
    release: bool,
}

#[derive(Args)]
struct RunArgs {
    #[command(flatten)]
    build_args: BuildArgs,

    /// Number of CPU cores
    #[clap(short, long, default_value_t = 4)]
    cores: usize,

    /// Redirect serial to stdio
    #[clap(short, long)]
    serial: bool,

    /// Wait for GDB attach (port 1234)
    #[clap(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        SubCommands::Build(args) => do_build(args).map(|_| ()),
        SubCommands::Run(args) => do_run(args),
        SubCommands::Clippy => do_clippy(),
        SubCommands::Test => do_test(),
    }
}

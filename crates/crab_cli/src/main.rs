extern crate clap;

use clap::{CommandFactory, Parser};
use crab_cli::CmdExecutor;

#[derive(clap::Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    debug: bool,
    #[command(subcommand)]
    command: Option<crab_cli::cli::SubCommand>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if args.debug {
        println!("{:?}", args);
    }

    match args.command {
        None => {
            Cli::command().print_help()?;
        }
        Some(cmd) => {
            cmd.execute().await?;
        }
    }

    Ok(())
}

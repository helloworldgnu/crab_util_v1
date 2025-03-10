extern crate clap;

extern crate enum_dispatch;

use clap::{CommandFactory, Parser};
use crab_cli::{Cli, CommandExecutor};

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

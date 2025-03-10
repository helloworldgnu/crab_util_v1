pub mod cmd_md5;

use enum_dispatch::enum_dispatch;

pub use cmd_md5::*;

#[derive(clap::Parser, Debug)]
#[command(name = "Cli",version, author, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub debug: bool,
    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(clap::Parser, Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum SubCommand {
    #[command(name = "md5")]
    Md5(cmd_md5::Md5),
}
mod cmd_md5;

use crate::CmdExecutor;
use enum_dispatch::enum_dispatch;

#[derive(clap::Subcommand, Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum SubCommand {
    Md5(cmd_md5::Md5),
}

impl CmdExecutor for SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Md5(cmd) => cmd.execute().await?,
        }
        Ok(())
    }
}

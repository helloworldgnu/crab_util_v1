use enum_dispatch::enum_dispatch;

pub mod cli;

#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}

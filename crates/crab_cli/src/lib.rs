pub mod cli;

pub use cli::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait CommandExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}

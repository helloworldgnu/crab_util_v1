pub mod http_serve;

use crate::CommandExecutor;
use enum_dispatch::enum_dispatch;

pub use http_serve::*;

use crate::cli::util::verify_path;

#[derive(clap::Parser, Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum HttpSubCommand {
    #[command(name = "serve")]
    Serve(HttpServe),
}

#[derive(clap::Parser, Debug)]
pub struct HttpServe {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: std::path::PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CommandExecutor for HttpServe {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port).await
    }
}

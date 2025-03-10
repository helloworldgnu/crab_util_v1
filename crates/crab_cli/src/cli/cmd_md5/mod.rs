use std::path::PathBuf;
use clap::CommandFactory;
use crate::{Cli, CommandExecutor};

#[derive(clap::Parser, Debug)]
#[group(required = false, multiple = false)]
pub struct Md5 {
    /// Text to calculate MD5 hash
    #[arg(short, long)]
    text: Option<String>,
    /// File to calculate MD5 hash
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// If neither text nor file is specified, the default input is treated as a file path.
    #[arg()]
    default_file: Option<PathBuf>,
}

impl CommandExecutor for Md5 {
    async fn execute(&self) -> anyhow::Result<()> {
        if let Some(text) = &self.text {
            println!("MD5 of text: {:x?}", md5::compute(text));
        } else if let Some(file) = &self.file {
            let content = std::fs::read(file).expect("Unable to read file");
            println!("MD5 of file: {:x?}", md5::compute(content));
        } else if let Some(default_file) = &self.default_file {
            let content = std::fs::read(default_file).expect("Unable to read file");
            println!("MD5 of file: {:x?}", md5::compute(content));
        } else {
            Cli::command().print_help()?;
        }

        Ok(())
    }
}

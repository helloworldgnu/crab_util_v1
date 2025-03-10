use crate::CmdExecutor;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Md5 {
    #[clap(flatten)]
    args: Md5Args,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Md5Args {
    /// Text to calculate MD5 hash
    #[clap(short, long)]
    text: Option<String>,
    /// File to calculate MD5 hash
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// If neither text nor file is specified, the default input is treated as a file path.
    #[clap()]
    default_file: Option<PathBuf>,
}

impl CmdExecutor for Md5 {
    async fn execute(&self) -> anyhow::Result<()> {
        if let Some(text) = &self.args.text {
            println!("MD5 of text: {:x?}", md5::compute(text));
        } else if let Some(file) = &self.args.file {
            let content = std::fs::read(file).expect("Unable to read file");
            println!("MD5 of file: {:x?}", md5::compute(content));
        } else if let Some(default_file) = &self.args.default_file {
            let content = std::fs::read(default_file).expect("Unable to read file");
            println!("MD5 of file: {:x?}", md5::compute(content));
        } else {
            eprintln!("No input provided");
        }

        Ok(())
    }
}

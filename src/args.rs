use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Web Portal Lite")]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[clap(about = "Start serving the app")]
    Serve,
    #[clap(about = "Hash a password for use in the user config")]
    PwHasher,
    #[clap(about = "Show app version")]
    Version,
}

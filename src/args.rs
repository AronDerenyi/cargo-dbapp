use clap::Parser;

#[derive(clap::Args)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(clap::Subcommand)]
pub enum Cmd {
    Build,
}

pub fn parse_args() -> Args {
    let Cargo::DBApp(args) = Cargo::parse();
    return args;
}

#[derive(clap::Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    #[command(name = "dbapp")]
    DBApp(Args),
}

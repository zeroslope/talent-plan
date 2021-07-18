use std::process::exit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    rename_all = "kebab-case", 
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Get the string value of a given string key")]
    Get { key: String },
    #[structopt(about = "Set the value of a string key to a string")]
    Set { key: String, value: String },
    #[structopt(name = "rm", about = "Remove a given key")]
    Remove { key: String },
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Set { key, value } => unimplemented(),
        Command::Get { key } => unimplemented(),
        Command::Remove { key } => unimplemented(),
    }
}

fn unimplemented() {
    eprintln!("unimplemented");
    exit(1);
}

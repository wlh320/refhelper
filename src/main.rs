use refhelper::{cli, Library};
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Start interactive CLI
    #[structopt(name = "cli")]
    Cli {
        /// path of library
        #[structopt(parse(from_os_str))]
        lib: Option<PathBuf>,
    },

    /// Generate bibtex file from library
    #[structopt(name = "gen")]
    Gen {
        /// path of library
        #[structopt(parse(from_os_str))]
        lib: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "refhelper", about = "A CLI tool to manage paper references")]
struct ArgCommand {
    #[structopt(subcommand)]
    sub: Option<SubCommand>,
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args = ArgCommand::from_args();
    match args.sub {
        Some(SubCommand::Cli { lib }) => {
            cli::loop_run(lib)?;
        }
        Some(SubCommand::Gen { lib }) => {
            Library::from_path(lib)?.gen_bibtex(None);
        }
        None => cli::loop_run(None)?,
    }
    Ok(())
}

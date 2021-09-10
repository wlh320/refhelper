use refhelper::{cli, Library};
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "refhelper", about = "A CLI tool to manage paper references")]
pub enum ArgCommand {
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
// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args = ArgCommand::from_args();
    match args {
        ArgCommand::Cli { lib } => {
            cli::loop_run(lib)?;
        }
        ArgCommand::Gen { lib } => {
            Library::from_path(lib)?.gen_bibtex();
        }
    }
    Ok(())
}

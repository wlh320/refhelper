use clap::AppSettings;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::Library;

const CLI_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::NoBinaryName,
    AppSettings::ColoredHelp,
    AppSettings::DisableVersion,
    AppSettings::DisableHelpFlags,
    AppSettings::VersionlessSubcommands,
];

#[derive(StructOpt, Debug)]
#[structopt(settings(CLI_CLAP_SETTINGS))]
#[structopt(usage = "Type command in an interactive shell")]
pub enum Command {
    /// Open a library
    #[structopt(name = "open")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Open {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },

    /// List entries of current library
    #[structopt(name = "list", alias = "ls")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    List,

    /// Add an Entry to this library
    #[structopt(name = "add")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Add { name: String, doi: String },

    /// Delete an Entry in this library
    #[structopt(name = "del", alias = "rm")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Del { id: usize },

    /// Create Link from entry and pdf file
    #[structopt(name = "link")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Link { id: usize, path: PathBuf },

    /// View chosen pdf file in pdfviewer
    #[structopt(name = "view")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    View { id: usize },

    /// Generate bibtex file of current library
    #[structopt(name = "gen")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Gen,

    /// Quit from interactive CLI
    #[structopt(name = "quit", alias = "exit")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Quit,
}

fn welcome() {
    println!("Welcome to refhelper {}!", env!("CARGO_PKG_VERSION"));
}

pub fn loop_run(libpath: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    welcome();
    // check if library is specified
    let mut lib = match libpath {
        Some(path) => Library::from_path(path)?,
        None => Library::new(),
    };
    // TODO: rustyline completion
    let mut rl = Editor::<()>::new();
    let prompt = ">> ";
    loop {
        let readline = rl.readline(prompt);
        let line = match readline {
            Ok(line) if line.trim() == "" => continue,
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(_) => String::from("help"),
        };
        match Command::from_iter_safe(line.split(' ')) {
            Ok(Command::Open { path }) => {
                lib = Library::from_path(path)?;
            }
            Ok(Command::Quit) => break,
            Ok(command) => {
                execute_command(&mut lib, command);
            }
            Err(e) => {
                println!("{}", e.message);
            }
        };
    }
    lib.save()?;
    Ok(())
}

fn execute_command(lib: &mut Library, command: Command) {
    if let None = lib.path {
        println!("No library is open");
        return;
    }
    match command {
        Command::List => lib.print(),
        Command::Add { name, doi } => lib.add(&name, &doi),
        Command::Del { id } => lib.del(id),
        Command::Link { id, path } => lib.link(id, path),
        Command::View { id } => lib.view(id),
        Command::Gen => lib.gen_bibtex(),
        _ => {}
    };
}

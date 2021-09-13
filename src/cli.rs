use clap::AppSettings;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use crate::rustyline;
use crate::utils;
use crate::Library;

const CLI_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::NoBinaryName,
    AppSettings::ColoredHelp,
    AppSettings::DisableVersion,
    AppSettings::DisableHelpFlags,
    AppSettings::VersionlessSubcommands,
    AppSettings::DeriveDisplayOrder,
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

    /// Search entries with some pattern
    #[structopt(name = "search", alias = "s")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Search {
        pat: String,
        #[structopt(short, long)]
        fuzzy: bool,
    },

    /// Add an entry to current library
    #[structopt(name = "add")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Add { name: String, doi: String },

    /// Add a batch of entries to current library (from a txt file)
    #[structopt(name = "add_batch")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    AddBatch {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },

    /// Load a batch of entries to current library (from a bibtex file)
    #[structopt(name = "load")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Load {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },

    /// Delete an entry in current library
    #[structopt(name = "del", alias = "rm")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Del { id: usize },

    /// Create link from entry to a pdf file
    #[structopt(name = "link")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Link { id: usize, path: PathBuf },

    /// View chosen pdf file in pdfviewer
    #[structopt(name = "view")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    View { id: usize },

    /// Generate bibtex file of one entry or entire library
    #[structopt(name = "gen")]
    #[structopt(settings(CLI_CLAP_SETTINGS))]
    Gen { id: Option<usize> },

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
        None => Library::default(),
    };
    let mut rl = rustyline::my_editor();
    let prompt = ">> ";
    loop {
        let readline = rl.readline(prompt);
        let line = match readline {
            Ok(line) if line.trim() == "" => continue,
            Ok(line) => line,
            Err(rustyline::ReadlineError::Interrupted) => break,
            Err(rustyline::ReadlineError::Eof) => break,
            Err(_) => String::from("help"),
        };
        match Command::from_iter_safe(line.trim().split_whitespace()) {
            Ok(Command::Open { path }) => lib = Library::from_path(path)?,
            Ok(Command::Quit) => break,
            Ok(command) => execute_command(&mut lib, command),
            Err(e) => println!("{}", e.message),
        };
        rl.add_history_entry(line);
    }
    lib.save()?;
    Ok(())
}

fn execute_command(lib: &mut Library, command: Command) {
    if lib.path.is_none() {
        println!("No library is open");
        return;
    }
    match command {
        Command::List => lib.print(),
        Command::Search { pat, fuzzy } => lib.search(&pat, fuzzy),
        Command::Add { name, doi } => lib.add(&name, &doi),
        Command::AddBatch { path } => lib.add_batch(utils::read_doi_file(path)),
        Command::Load { path } => lib.load_bibtex(utils::read_bibtex_file(path)),
        Command::Del { id } => lib.del(id),
        Command::Link { id, path } => lib.link(id, path),
        Command::View { id } => lib.view(id),
        Command::Gen { id } => lib.gen_bibtex(id),
        _ => {}
    };
}

use biblatex::{self, Bibliography, ChunksExt};
use futures::{stream, StreamExt};
use comfy_table::{Table, ContentArrangement};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::{self, Stdio};
use std::{error::Error, path::PathBuf};
use tokio::runtime::Runtime;

pub mod cli;
mod utils;
mod rustyline;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    name: String,
    doi: String,
    bibtex: String,
    title: String,
    path: Option<PathBuf>,
    note: String,
}

impl Entry {
    pub fn new(name: &str, doi: &str) -> Entry {
        Entry {
            name: String::from(name),
            doi: String::from(doi),
            bibtex: String::from(""),
            path: None,
            title: String::from(""),
            note: String::from(""),
        }
    }

    pub fn link(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub async fn get_bib(&mut self) -> Result<(), Box<dyn Error>> {
        let bib = utils::doi2bib(&self.doi).await?;
        self.parse_bibtex(bib)?;
        Ok(())
    }

    fn parse_bibtex(&mut self, bib: String) -> Result<(), String> {
        let mut bibs = Bibliography::parse(&bib).unwrap(); // only one entry
        match bibs.iter_mut().next() {
            Some(entry) => {
                // update entry cite key to entry.name
                entry.key = self.name.clone();
                self.title = entry.title().unwrap().format_sentence();
                self.bibtex = entry.to_bibtex_string();
                Ok(())
            }
            None => Err(String::from("Invalid DOI")),
        }
    }

    pub fn take_note(&mut self, note: &str) {
        self.note = String::from(note);
    }

    pub fn print(&self) {
        println!("name: {} title: {}", self.name, self.title);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    path: Option<PathBuf>,
    entries: Vec<Entry>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            path: None,
            entries: vec![],
        }
    }

    pub fn from_path(path: PathBuf) -> Result<Library, Box<dyn Error>> {
        let lib = match File::open(&path) {
            Ok(file) => serde_json::from_reader(file)?,
            Err(_) => {
                File::create(&path)?;
                Library {
                    path: Some(path),
                    entries: vec![],
                }
            }
        };
        println!("Open library {}", &lib.path.as_ref().unwrap().display());
        Ok(lib)
    }

    pub fn add(&mut self, name: &str, doi: &str) {
        let mut new_entry = Entry::new(name, doi);
        let rt = Runtime::new().unwrap();
        match rt.block_on(new_entry.get_bib()) {
            Ok(()) => self.entries.push(new_entry),
            Err(e) => println!("Failed to add entry, error: {}", e),
        };
    }

    pub fn add_batch(&mut self, mut entries: Vec<Entry>) {
        const CONCURRENT_NUM: usize = 5;
        let rt = Runtime::new().unwrap();
        let results = rt.block_on(async {
            let tasks = entries.iter_mut().map(|e| e.get_bib());
            stream::iter(tasks)
                .map(|task| async { task.await })
                .buffered(CONCURRENT_NUM)
                .collect::<Vec<_>>()
                .await
        });
        let old_len = self.entries.len();
        for (entry, result) in entries.into_iter().zip(results.into_iter()) {
            match result {
                Ok(()) => self.entries.push(entry),
                Err(e) => println!("{} error: {}", entry.name, e),
            }
        }
        println!("Read {} entries from file", self.entries.len() - old_len);
    }

    pub fn del(&mut self, id: usize) {
        if id < self.entries.len() {
            self.entries.remove(id);
        } else {
            println!("No such id");
        }
    }

    pub fn link(&mut self, id: usize, path: PathBuf) {
        // TODO: download pdf automaticlly
        match self.entries.get_mut(id) {
            None => println!("No such id"),
            Some(e) => e.link(path),
        };
    }

    pub fn view(&self, id: usize) {
        match self.entries.get(id) {
            None => println!("No such id"),
            Some(entry) => match &entry.path {
                None => println!("No pdf file of this entry"),
                Some(p) => {
                    // TODO: cross-platform
                    process::Command::new("xdg-open")
                        .arg(p.as_os_str())
                        .stdout(Stdio::null())
                        .spawn()
                        .expect("Failed to open pdf file");
                }
            },
        };
    }

    pub fn print(&self) {
        if let Some(p) = &self.path {
            println!("Current library: {}", p.display());
        };
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec!["id", "name", "title", "doi", "path"]);
        for (id, entry) in self.entries.iter().enumerate() {
            table.add_row(vec![
                &id.to_string(),
                &entry.name,
                &entry.title,
                &entry.doi,
                &match entry.path {
                    Some(_) => "y".to_string(),
                    None => "n".to_string(),
                }
            ]);
        }
        println!("{}", table);
    }

    // TODO: fuzzy search from library

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.path {
            serde_json::to_writer(&File::create(p)?, self)?;
        }
        Ok(())
    }

    pub fn gen_bibtex(&self) {
        for entry in self.entries.iter() {
            println!("{}", entry.bibtex);
        }
    }

    pub fn load_bibtex(&mut self, entries: Vec<Entry>) {
        let len = entries.len();
        self.entries.extend(entries.into_iter());
        println!("load {} entries from file", len);
    }
}

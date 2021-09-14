use biblatex::{self, Bibliography, ChunksExt};
use futures::{stream, StreamExt};
use fuzzy_matcher::skim::SkimMatcherV2;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use tokio::runtime::Runtime;

pub mod cli;
pub mod downloader;
mod rustyline;
mod utils;

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
            doi: String::from(doi), // if arxiv paper, it is arxiv id
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
        if self.doi.starts_with("10.") {
            let bib = downloader::DOIDownloader::get_bibtex(&self.doi).await?;
            self.parse_bibtex(bib)?;
        } else {
            let bib = downloader::ArxivDownloader::get_bibtex(&self.doi).await?;
            self.parse_bibtex(bib)?;
        }
        Ok(())
    }

    fn parse_bibtex(&mut self, bib: String) -> Result<(), String> {
        let mut bibs = Bibliography::parse(&bib).unwrap();
        // only one entry
        match bibs.iter_mut().next() {
            Some(e) => {
                // update entry cite key to entry.name
                if e.key.is_empty() {
                    return Err(String::from("Failed to get bibtex from DOI/arXiv"));
                }
                e.key = self.name.clone();
                self.title = e.title().map_or("".to_owned(), |t| t.format_sentence());
                self.bibtex = e.to_bibtex_string();
                Ok(())
            }
            None => Err(String::from("Failed to get bibtex from DOI/arXiv")),
        }
    }

    pub fn take_note(&mut self, note: &str) {
        self.note = String::from(note);
    }

    pub fn print(&self) {
        println!("name: {} title: {}", self.name, self.title);
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Library {
    entries: Vec<Entry>,

    #[serde(skip)]
    path: Option<PathBuf>,
}

impl Library {
    pub fn from_path(path: PathBuf) -> Result<Library, Box<dyn Error>> {
        let lib = match File::open(&path) {
            Ok(file) => {
                let mut exist_lib: Library = serde_json::from_reader(file)?;
                exist_lib.path = Some(path);
                exist_lib
            }
            Err(_) => {
                File::create(&path)?;
                Library {
                    path: Some(path),
                    entries: vec![],
                }
            }
        };
        Ok(lib)
    }

    pub fn add(&mut self, name: &str, id: &str) {
        let mut new_entry = Entry::new(name, id);
        let rt = Runtime::new().unwrap();
        match rt.block_on(new_entry.get_bib()) {
            Ok(()) => self.entries.push(new_entry),
            Err(e) => println!("Failed to add entry, error: {}", e),
        };
    }

    pub fn add_batch(&mut self, mut entries: Vec<Entry>) {
        const CONCURRENT_NUM: usize = 5;
        let pb = ProgressBar::new(entries.len() as u64);
        let rt = Runtime::new().unwrap();
        let results = rt.block_on(async {
            let tasks = entries.iter_mut().map(|e| e.get_bib());
            stream::iter(tasks)
                .map(|task| async {
                    let r = task.await;
                    pb.inc(1);
                    r
                })
                .buffered(CONCURRENT_NUM)
                .collect::<Vec<_>>()
                .await
        });
        pb.finish_with_message("done");
        let old_len = self.entries.len();
        for (entry, result) in entries.into_iter().zip(results.into_iter()) {
            match result {
                Ok(()) => self.entries.push(entry),
                Err(e) => println!("add '{}' error: {}", entry.name, e),
            }
        }
        println!("Add {} entries from file", self.entries.len() - old_len);
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
                    if let Err(e) = utils::view_pdf_file(p) {
                        println!("View pdf file failed, error {}", e);
                    }
                }
            },
        };
    }

    pub fn print(&self) {
        if let Some(p) = &self.path {
            println!("Current library: {}", p.display());
        };
        utils::print_entries(&mut self.entries.iter().enumerate());
    }

    pub fn search(&self, pat: &str, fuzzy: bool) {
        if let Some(p) = &self.path {
            println!("Current library: {}", p.display());
        };
        let matcher: Box<dyn utils::Matcher> = if fuzzy {
            Box::new(SkimMatcherV2::default())
        } else {
            Box::new(utils::StrictMatcher)
        };
        let mut matched = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(id, e)| matcher.score(&e.bibtex, pat).map(|s| (s, id, e)))
            .collect::<Vec<_>>();
        matched.sort_by_key(|t| -t.0);
        let mut to_print = matched.into_iter().map(|(_, i, e)| (i, e));
        utils::print_entries(&mut to_print);
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.path {
            serde_json::to_writer(&File::create(p)?, self)?;
        }
        Ok(())
    }

    pub fn gen_bibtex(&self, id: Option<usize>) {
        match id {
            None => {
                // gen all bibtex
                for entry in self.entries.iter() {
                    println!("{}", entry.bibtex);
                }
            }
            Some(i) => {
                // gen for one entry
                let output = self.entries.get(i).map_or("No such id", |e| &e.bibtex);
                println!("{}", output);
            }
        }
    }

    pub fn load_bibtex(&mut self, entries: Vec<Entry>) {
        let len = entries.len();
        self.entries.extend(entries.into_iter());
        println!("load {} entries from file", len);
    }

    pub fn download(&mut self, folder: Option<PathBuf>) {
        let path = match folder {
            Some(p) => p,
            None => self.path.clone().unwrap().parent().unwrap().to_path_buf(),
        };
        let ids: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.path.is_none())
            .map(|e| &e.doi[..])
            .collect();
        let rt = Runtime::new().unwrap();
        if let Err(e) = rt.block_on(downloader::download_pdfs(ids, path.clone())) {
            println!("{}", e);
        }
        self.entries
            .iter_mut()
            .filter(|e| e.path.is_none())
            .for_each(|e| {
                let mut file = path.clone();
                file.push(format!("{}.pdf", e.doi));
                if file.exists() {
                    e.link(file);
                }
            });
    }
}

use biblatex::{self, Bibliography, ChunksExt};
use comfy_table::{ContentArrangement, Table};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use indicatif::ProgressIterator;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

use crate::Entry;

pub fn read_doi_file(path: PathBuf) -> Vec<Entry> {
    // file format example:
    // name1 doi1 [pdf filepath]
    // name2 doi2 [pdf filepath]
    // ...   ...
    let mut entries: Vec<Entry> = Vec::new();
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Read file failed, error: {}", e);
            String::new()
        }
    };
    for (lineno, line) in content.lines().enumerate() {
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        match words.len() {
            2 => entries.push(Entry::new(words[0], words[1])),
            3 => {
                let mut entry = Entry::new(words[0], words[1]);
                entry.link(PathBuf::from(words[2]));
                entries.push(entry);
            }
            _ => println!("error read doi file at line {}", lineno),
        };
    }
    entries
}

pub fn read_bibtex_file(path: PathBuf) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Read file failed, error: {}", e);
            String::new()
        }
    };
    let bibs = Bibliography::parse(&content).unwrap();
    for e in bibs.into_iter().progress() {
        let name = &e.key;
        let arxiv_id = e.eprint().unwrap_or_default();
        let doi = &e.doi().unwrap_or(arxiv_id);
        // parse
        let mut entry = Entry::new(name, doi);
        entry.bibtex = e.to_bibtex_string();
        entry.title = e.title().unwrap_or_default().format_sentence();
        entries.push(entry);
    }
    entries
}

pub fn view_pdf_file(path: &Path) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    process::Command::new("xdg-open")
        .arg(path.as_os_str())
        .stdout(Stdio::null())
        .spawn()?;

    #[cfg(target_os = "windows")]
    process::Command::new("cmd")
        .args(["/C", "start"])
        .arg(path.as_os_str())
        .stdout(Stdio::null())
        .spawn()?;

    Ok(())
}

pub fn print_entries(entries: &mut dyn Iterator<Item = (usize, &Entry)>) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["id", "name", "title", "doi", "path"]);
    for (id, entry) in entries {
        table.add_row(vec![
            &id.to_string(),
            &entry.name,
            &entry.title,
            &entry.doi,
            &match entry.path {
                Some(_) => "y".to_string(),
                None => "n".to_string(),
            },
        ]);
    }
    println!("{}", table);
}

// simple search, no fuzzy, count as score
pub trait Matcher {
    fn score(&self, choice: &str, pattern: &str) -> Option<i64>;
}
impl Matcher for SkimMatcherV2 {
    fn score(&self, choice: &str, pattern: &str) -> Option<i64> {
        self.fuzzy_match(choice, pattern)
    }
}
pub struct StrictMatcher;
impl Matcher for StrictMatcher {
    fn score(&self, choice: &str, pattern: &str) -> Option<i64> {
        match choice.matches(pattern).count() {
            0 => None,
            x => Some(x as i64),
        }
    }
}

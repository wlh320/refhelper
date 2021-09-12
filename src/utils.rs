use biblatex::{self, Bibliography, ChunksExt};
use comfy_table::{ContentArrangement, Table};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use reqwest;
use std::error::Error;
use std::path::PathBuf;
use std::process::{self, Stdio};
use std::vec;

use crate::Entry;

pub async fn doi2bib(doi: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://doi.org/{}", doi);
    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("Accept", "application/x-bibtex; charset=utf-8")
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

pub fn read_doi_file(path: PathBuf) -> Vec<Entry> {
    // file format example:
    // name1 doi1
    // name2 doi2
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
        if words.len() != 2 {
            println!("error read doi file at line {}", lineno);
            continue;
        }
        entries.push(Entry::new(words[0], words[1]));
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
    for e in bibs {
        let name = &e.key;
        let doi = &e.doi().unwrap_or(String::from(""));
        // parse
        let mut entry = Entry::new(&name, &doi);
        entry.bibtex = e.to_bibtex_string();
        entry.title = match e.title() {
            None => String::new(),
            Some(t) => t.format_sentence(),
        };
        entries.push(entry);
    }
    entries
}

pub fn view_pdf_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
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
        return self.fuzzy_match(choice, pattern);
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

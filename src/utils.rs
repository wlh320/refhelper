use biblatex::{self, Bibliography, ChunksExt};
use std::path::PathBuf;

use reqwest;

use crate::Entry;

pub async fn doi2bib(doi: &str) -> Result<String, Box<dyn std::error::Error>> {
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

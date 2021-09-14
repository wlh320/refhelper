use futures::{stream, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use scraper::{Html, Selector};
use std::{error::Error, fs::File, io::Write, path::PathBuf};

pub struct Downloader;

impl Downloader {
    pub async fn get_pdf(id: &str, path: PathBuf, pb: &ProgressBar) -> Result<(), Box<dyn Error>> {
        if id.starts_with("10.") {
            pb.set_length(0);
            pb.set_message("not impl skip");
            pb.finish();
        } else {
            ArxivDownloader::get_pdf(id, path, pb).await?;
        }
        Ok(())
    }
}
pub struct ArxivDownloader;

impl ArxivDownloader {
    async fn arxiv2doi(aid: &str) -> Result<Option<String>, Box<dyn Error>> {
        let url = format!("https://arxiv.org/abs/{}", aid);
        let client = reqwest::Client::builder()
            .user_agent("hyper/0.5.2".to_owned())
            .build()?;
        let body = client.get(url).send().await?.text().await?;
        let selector = Selector::parse(r#"[name="citation_doi"]"#).unwrap();
        let doi = Html::parse_fragment(&body)
            .select(&selector)
            .next()
            .and_then(|ele| ele.value().attr("content").map(|s| s.into()));
        Ok(doi)
    }

    pub async fn get_bibtex(id: &str) -> Result<String, Box<dyn Error>> {
        let doi: Option<String> = Self::arxiv2doi(id).await?;
        if doi.is_none() {
            let url = format!("https://arxiv.org/bibtex/{}", id);
            let client = reqwest::Client::builder()
                .user_agent("hyper/0.5.2".to_owned())
                .build()?;
            let body = client
                .get(url)
                .header("Accept", "text/bibliography; style=bibtex")
                .send()
                .await?
                .text()
                .await?;
            Ok(body)
        } else {
            let body = DOIDownloader::get_bibtex(doi.as_deref().unwrap()).await?;
            Ok(body)
        }
    }

    pub async fn get_pdf(id: &str, path: PathBuf, pb: &ProgressBar) -> Result<(), Box<dyn Error>> {
        let url = format!("https://arxiv.org/pdf/{}", id);
        let client = reqwest::Client::builder()
            .user_agent("hyper/0.5.2".to_owned())
            .build()?;
        let res = client
            .get(url)
            .header("Accept", "text/bibliography; style=bibtex")
            .send()
            .await?;

        let total_size = res.content_length().ok_or("Failed to get file length")?;
        pb.set_length(total_size);
        pb.set_message(format!("{}.pdf", id));

        let mut file = File::create(path)?;
        let mut downloaded: usize = 0;
        let mut stream = res.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            let len = file.write(&chunk)?;
            downloaded += len;
            pb.set_position(downloaded as u64);
        }
        pb.finish();
        Ok(())
    }
}
pub struct DOIDownloader;

impl DOIDownloader {
    pub async fn get_bibtex(doi: &str) -> Result<String, Box<dyn Error>> {
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
}

fn set_pb_style(pb: &ProgressBar) {
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg:14} [{elapsed_precise}] [{bar:40}] {bytes:>7}/{total_bytes:7}")
            .progress_chars("##-"),
    );
}

pub async fn download_pdfs(ids: Vec<&str>, path: PathBuf) -> Result<(), Box<dyn Error>> {
    const CONCURRENT_NUM: usize = 5;
    let m = MultiProgress::new();
    let total_pb = m.add(ProgressBar::new(ids.len() as u64));
    total_pb.set_message("downloading");
    total_pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg:14} [{elapsed_precise}] [{bar:40}] {pos:>7}/{len:7}")
            .progress_chars("##-"),
    );
    let pbs: Vec<_> = (0..ids.len())
        .map(|i| m.insert(i, ProgressBar::new(1)))
        .collect();
    let tasks = ids.iter().enumerate().map(|(i, id)| {
        let mut p = path.clone();
        p.push(format!("{}.pdf", id));
        set_pb_style(&pbs[i]);
        Downloader::get_pdf(id, p, &pbs[i])
    });

    let handle_m = tokio::task::spawn_blocking(move || m.join().unwrap());
    stream::iter(tasks)
        .map(|task| async {
            let r = task.await;
            total_pb.inc(1);
            r
        })
        .buffered(CONCURRENT_NUM)
        .collect::<Vec<_>>()
        .await;
    total_pb.finish_with_message("done");
    handle_m.await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn test_arxiv2doi() -> Result<(), Box<dyn Error>> {
        let case1 = ArxivDownloader::arxiv2doi("2109.03989").await?;
        let case2 = ArxivDownloader::arxiv2doi("1703.05907").await?;
        assert_eq!(case1, Some("10.1109/CSR51186.2021.9527910".to_owned()));
        assert_eq!(case2, None);
        Ok(())
    }
}

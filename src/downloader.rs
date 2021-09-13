use scraper::{Html, Selector};
use std::{error::Error, path::Path};

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

    pub async fn _get_pdf(id: &str, _path: &Path) -> Result<(), Box<dyn Error>> {
        let _url = format!("https://arxiv.org/pdf/{}", id);
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

    // async fn get_pdf(id: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    //     Ok(())
    // }
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

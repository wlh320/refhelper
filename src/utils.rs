use reqwest;

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

use error_chain::error_chain;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use select::document::Document;
use select::predicate::{And, Child, Class, Name};

error_chain! {
    foreign_links{
        ReqError(reqwest::Error);
IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    scrape_imdb_titles().await?;

    Ok(())
}

async fn scrape_imdb_titles() -> Result<()> {
    let url = "https://www.imdb.com/search/title/";
    let mut headers = HeaderMap::new();
    headers.insert(
        "Sec-Ch-Ua",
        HeaderValue::from_static(
            r##""Chromium";v="130", "Google Chrome";v="130", "Not?A_Brand";v="99""##,
        ),
    );
    let client = Client::builder().default_headers(headers).user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36").https_only(true).build()?;
    // ?groups=top_100&sort=user_rating,desc&count=100
    let req = client
        .get(url)
        .query(&[
            ("groups", "top_100"),
            ("sort", "user_rating,desc"),
            ("count", "100"),
        ])
        .build()?;

    let response = client.execute(req).await?.text().await?;

    // let text = response
    //     .text()
    //     .map_err(|e| anyhow!("Failed to read response text: {e}"))?;

    let document = Document::from(response.as_str());

    let nodes: Vec<_> = document
        // .find(And(Name("h3"), Class("ipc-title__text")))
        .find(Child(
            And(Name("a"), Class("ipc-title-link-wrapper")),
            Name("h3"),
        ))
        .collect();

    let titles: Vec<_> = nodes.iter().map(|n| n.inner_html()).collect();

    println!("nodes: {}, names: {}", nodes.len(), titles.len());

    for title in titles.iter().take(100) {
        println!("{title}");
    }

    Ok(())
}

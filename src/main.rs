use anyhow::{anyhow, Result};
use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    let scraping_result = scrape_imdb_titles();
    if let Err(e) = scraping_result {
        eprintln!("Error: {e}");
    }
}

fn scrape_imdb_titles() -> Result<()> {
    let url = "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100";

    let response = get(url).map_err(|e| anyhow!("Failed to fetch URL: {e}"))?;

    let text = response
        .text()
        .map_err(|e| anyhow!("Failed to read response text: {e}"))?;

    let document = Html::parse_document(&text);

    let title_selector = Selector::parse("a.ipc-title-link-wrapper>h3")
        .map_err(|e| anyhow!("Failed to parse selector: {e}"))?;

    let titles: Vec<_> = document
        .select(&title_selector)
        .map(|x| x.inner_html())
        .collect();

    println!("Displaying 100 of {} ", titles.len());
    //
    // titles
    //     .iter()
    //     .take(100)
    //     .enumerate()
    //     .for_each(|(index, title)| println!("{}. {}", index + 1, title));

    for title in titles.iter().take(100) {
        println!("{title}");
    }

    Ok(())
}

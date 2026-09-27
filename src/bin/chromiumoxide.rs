use chromiumoxide::browser::{Browser, BrowserConfig};
use futures_lite::stream::StreamExt;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
    tokio::spawn(async move { while handler.next().await.is_some() {} });

    let page = browser.new_page("https://github.com/search").await?;
    page.find_element("[aria-label='Search GitHub']")
        .await?
        .click().await?
        .type_str("qa.guru").await?
        .press_key("Enter").await?;

    let mut results = String::new();
    for _ in 0..30 {
        if let Ok(el) = page.find_element("[data-testid='results-list']").await {
            if let Ok(t) = el.inner_text().await {
                results = t.unwrap_or_default();
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    assert!(results.contains("QA.GURU"), "expected QA.GURU in results, got: {results}");

    browser.close().await?;
    Ok(())
}

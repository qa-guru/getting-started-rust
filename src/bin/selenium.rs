use std::process::{Command, Stdio};
use std::time::Duration;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = Command::new("chromedriver")
        .arg("--port=9515")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("chromedriver not found on PATH (brew install chromedriver)");
    tokio::time::sleep(Duration::from_millis(800)).await;

    let driver = WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome()).await?;
    driver.goto("https://github.com/search").await?;
    driver
        .query(By::Css("[aria-label='Search GitHub']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys("qa.guru\u{e007}")
        .await?;

    let text = driver
        .query(By::Css("[data-testid='results-list']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .text()
        .await?;
    assert!(text.contains("QA.GURU"), "expected QA.GURU in results, got: {text}");

    driver.quit().await?;
    service.kill()?;
    Ok(())
}

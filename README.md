# Getting started with Rust UI tests

Two ways to do the same UI test: open GitHub search, search for `qa.guru`, assert results contain `QA.GURU`.

- `src/bin/selenium.rs` — [thirtyfour](https://github.com/Vrtgs/thirtyfour) WebDriver client (needs `chromedriver` on PATH, e.g. `brew install chromedriver`)
- `src/bin/chromiumoxide.rs` — [chromiumoxide](https://github.com/mattsse/chromiumoxide), Chrome DevTools Protocol client

## Run

```bash
cargo run --bin selenium
cargo run --bin chromiumoxide
```

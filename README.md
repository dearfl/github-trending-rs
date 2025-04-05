# GitHub Trending Repositories in Rust

A simple Rust crate to fetch trending repositories from GitHub.

## Usage

### Example: Get daily Rust trending repositories

```rust
use std::time::Duration;

use github_trending_rs::{Client, Language, Since};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;

    let client = Client::with_client(client);

    for repo in client
        .trending()
        .with_language(Language::Rust)
        .since(Since::Daily)
        .repositories()
        .await?
        .iter() {
        println!("{:#?}", repo);
    }

    Ok(())
}
```

## Limitations

- This crate relies on GitHub's public API. Changes to GitHub's website or rate limiting policies may affect functionality.
- The HTML parsing is based on the current structure of GitHub's trending page, which could change over time.

## License

This project is licensed under the GLWT (Good Luck With That Public License). See the `LICENSE` file for more details.

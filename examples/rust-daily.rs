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
        .iter()
    {
        println!("{:#?}", repo);
    }
    Ok(())
}

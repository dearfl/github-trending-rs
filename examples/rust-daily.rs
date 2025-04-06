use std::{
    collections::HashSet,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use github_trending_rs::{Client, Language, Since};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let history: HashSet<String> = std::fs::read_dir("./archive")?
        .flat_map(|dir| {
            dir.ok().and_then(|dir| {
                std::fs::File::open(dir.path())
                    .ok()
                    .map(|f| std::io::BufReader::new(f))
                    .and_then(|reader| serde_json::from_reader(reader).ok())
            })
        })
        .flat_map(|data: Vec<String>| data.into_iter())
        .collect();
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;
    let client = Client::with_client(client);
    let trending = client
        .trending()
        .with_language(Language::Rust)
        .since(Since::Daily)
        .repositories()
        .await?;
    let escape = |out: &str| {
        let out = out.replace("-", "\\-");
        let out = out.replace("+", "\\+");
        let out = out.replace("_", "\\_");
        let out = out.replace("*", "\\*");
        let out = out.replace("~", "\\~");
        let out = out.replace("#", "\\#");
        let out = out.replace("=", "\\=");
        let out = out.replace(".", "\\.");
        let out = out.replace("!", "\\!");
        let out = out.replace("|", "\\|");
        let out = out.replace("(", "\\(");
        let out = out.replace(")", "\\)");
        let out = out.replace("[", "\\[");
        let out = out.replace("]", "\\]");
        let out = out.replace("{", "\\{");
        let out = out.replace("}", "\\}");
        let out = out.replace("`", "\\`");
        let out = out.replace(">", "\\>");
        out
    };
    for repo in trending.iter() {
        let ident = format!("{}/{}", repo.owner, repo.name);
        if history.contains(&ident) {
            continue;
        }
        println!(
            "[{}]({}): {}",
            escape(&repo.name),
            escape(&repo.url()),
            escape(&repo.description)
        );
    }
    let repos: Vec<_> = trending
        .iter()
        .map(|repo| format!("{}/{}", repo.owner, repo.name))
        .collect();
    let content = serde_json::to_string(&repos)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let path = format!("./archive/{}.json", now);
    std::fs::write(path, content)?;
    Ok(())
}

mod params;

use std::marker::PhantomData;

pub use params::{Language, Since, SpokenLanguage};

use soup::{NodeExt as _, QueryBuilderExt as _, prelude::Soup};
use thiserror::Error;

// TODO: trending developers?

#[derive(Clone, Debug)]
pub struct Repository {
    pub name: String,
    pub owner: String,
    pub description: String,
    // TODO: impl
    // pub stars: usize,
    // pub forks: usize,
    // pub stars_since: usize,
    // pub language: String,
    // pub contributors: Vec<String>,
}

impl Repository {
    pub fn url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.name)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn trending(&self) -> TrendingBuilder {
        TrendingBuilder {
            client: self.client.clone(),
            lang: None,
            spoken: None,
            since: None,
        }
    }
}

pub struct TrendingBuilder {
    client: reqwest::Client,
    lang: Option<Language>,
    spoken: Option<SpokenLanguage>,
    since: Option<Since>,
}

impl TrendingBuilder {
    pub fn with_language(self, lang: Language) -> Self {
        Self {
            lang: Some(lang),
            ..self
        }
    }

    pub fn since(self, since: Since) -> Self {
        Self {
            since: Some(since),
            ..self
        }
    }

    pub fn with_spoken_language(self, spoken_language: SpokenLanguage) -> Self {
        Self {
            spoken: Some(spoken_language),
            ..self
        }
    }

    pub async fn repositories(self) -> Result<Trending<Repository>, Error> {
        let url = match self.lang {
            Some(lang) => format!("https://github.com/trending/{}", lang.code()),
            None => "https://github.com/trending".to_string(),
        };
        let req = self.client.get(url);
        let req = match self.since {
            Some(since) => req.query(&[("since", since.code())]),
            None => req,
        };
        let req = match self.spoken {
            Some(spoken) => req.query(&[("spoken_language_code", spoken.code())]),
            None => req,
        };
        let resp = req.send().await?.text().await?;
        Ok(Trending {
            raw: resp,
            _t: PhantomData,
        })
    }
}

pub trait Extract: Sized {
    fn extract(text: &str) -> impl Iterator<Item = Self>;
}

impl Extract for Repository {
    fn extract(text: &str) -> impl Iterator<Item = Self> {
        let soup = Soup::new(text);
        soup.tag("article").into_iter().filter_map(|article| {
            // any failed parse is silently discarded
            let url = article.tag("h2").find().and_then(|node| {
                node.tag("a")
                    .find()
                    .and_then(|node| node.get("href"))
                    .and_then(|url| {
                        url.strip_prefix("/").and_then(|url| {
                            url.split_once("/")
                                .map(|(owner, name)| (owner.to_string(), name.to_string()))
                        })
                    })
            });
            let p = article
                .tag("p")
                .find()
                .map(|node| node.text().trim().to_string());
            url.zip(p).map(|((owner, name), description)| Repository {
                name,
                owner,
                description,
            })
        })
    }
}

pub struct Trending<T: Extract> {
    raw: String,
    _t: PhantomData<T>,
}

impl<T: Extract> Trending<T> {
    pub fn iter(&self) -> impl Iterator<Item = T> {
        T::extract(&self.raw)
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

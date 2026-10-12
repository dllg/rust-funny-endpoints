use rand::RngExt;

use crate::{client::HttpClient, error::Error};

/// One upstream API and where its text lives in the JSON (RFC 6901 pointer).
struct Source {
    name: &'static str,
    url: &'static str,
    headers: &'static [(&'static str, &'static str)],
    pointer: &'static str,
}

const SOURCES: [Source; 3] = [
    Source {
        name: "Advice",
        url: "https://api.adviceslip.com/advice",
        headers: &[],
        pointer: "/slip/advice",
    },
    Source {
        name: "Chuck Norris Joke",
        url: "https://api.chucknorris.io/jokes/random",
        headers: &[],
        pointer: "/value",
    },
    Source {
        name: "Dad Joke",
        url: "https://icanhazdadjoke.com/",
        headers: &[("Accept", "application/json")],
        pointer: "/joke",
    },
];

pub struct Funny<C> {
    client: C,
}

impl<C: HttpClient> Funny<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn advice(&self) -> Result<String, Error> {
        self.fetch(&SOURCES[0]).await
    }

    pub async fn chucknorris(&self) -> Result<String, Error> {
        self.fetch(&SOURCES[1]).await
    }

    pub async fn dadjoke(&self) -> Result<String, Error> {
        self.fetch(&SOURCES[2]).await
    }

    /// A message from a random upstream, prefixed with its kind.
    pub async fn random_message(&self) -> Result<String, Error> {
        let source = &SOURCES[rand::rng().random_range(0..SOURCES.len())];
        Ok(format!("{}: {}", source.name, self.fetch(source).await?))
    }

    async fn fetch(&self, source: &Source) -> Result<String, Error> {
        let json = self.client.get(source.url, source.headers).await?;
        json.pointer(source.pointer)
            .and_then(|v| v.as_str())
            .map(str::to_owned)
            .ok_or(Error::Payload(source.url))
    }
}

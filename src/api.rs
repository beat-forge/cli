use anyhow::Result;
use minreq::{get as mget, post as mpost, Error, Response};
use serde::{Deserialize, Serialize};

use crate::structs::{BeatSaberVersionResponse, CategoryResponse, GqlResponse, User};

pub struct Client {
    pub api_url: String,
    pub api_key: Option<String>,
}

impl Client {
    pub fn new(api_url: String, api_key: Option<String>) -> Self {
        Self { api_url, api_key }
    }

    fn get(&self, path: &str) -> Result<Response, Error> {
        let req = mget(format!("{}{}", self.api_url, path)).with_header(
            "User-Agent",
            format!("beatforge/{}", env!("CARGO_PKG_VERSION")),
        );

        if self.api_key.is_some() {
            req.with_header("Authorization", "Bearer ".to_string() + self.api_key.as_ref().unwrap())
                .send()
        } else {
            req.send()
        }
    }

    fn post(&self, path: &str, body: &str) -> Result<Response, Error> {
        let req = mpost(format!("{}{}", self.api_url, path))
            .with_header(
                "User-Agent",
                format!("beatforge/{}", env!("CARGO_PKG_VERSION")),
            )
            .with_header("Content-Type", "application/json")
            .with_body(body);

        if self.api_key.is_some() {
            req.with_header("Authorization", "Bearer ".to_string() + self.api_key.as_ref().unwrap())
                .send()
        } else {
            req.send()
        }
    }

    fn gql<U: for<'a> Deserialize<'a> + Serialize, T: Into<serde_json::Value>>(
        &self,
        query: T,
    ) -> Result<GqlResponse<U>> {
        let body = serde_json::json!({
            "query": query.into(),
        });

        let res = self.post("/graphql", &body.to_string()).unwrap();
        let body = res.as_str().unwrap();
        Ok(serde_json::from_str(body)?)
    }

    pub fn get_categories(&self) -> Result<Vec<String>> {
        Ok(self
            .gql::<CategoryResponse, _>(
                r#"
        {
            categories {
                name
            }
        }
        "#,
            )?
            .data
            .categories
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<_>>())
    }

    pub fn get_beat_saber_versions(&self) -> Result<Vec<String>> {
        Ok(self
            .gql::<BeatSaberVersionResponse, _>(
                r#"
        {
            beatSaberVersions
        }
        "#,
            )?
            .data
            .beat_saber_versions
            .into_iter()
            .collect::<Vec<_>>())
    }

    pub fn get_me(&self) -> Result<User> {
        Ok(self.get("/me")?.json::<User>()?)
    }
}

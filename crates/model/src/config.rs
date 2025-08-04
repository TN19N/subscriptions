use serde::Deserialize;
use std::fmt::Debug;
use url::Url;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub url: Url,
    pub username: String,
    pub password: String,
    pub namespace: Option<String>,
    pub name: Option<String>,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("url", &self.url.as_str())
            .field("username", &self.username)
            .field("password", &self.password)
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .finish()
    }
}

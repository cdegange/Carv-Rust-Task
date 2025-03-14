use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)] // Fix: Ensure `Serialize` is imported
pub struct Repository { // Fix: `pub` struct for visibility
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: u32,
}

pub fn fetch_repos(username: &str, token: Option<&str>) -> Vec<Repository> {
    let client = Client::new();
    let url = format!("https://api.github.com/users/{}/repos", username);

    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(t) = token {
        headers.insert("Authorization", format!("Bearer {}", t).parse().unwrap());
    }

    let response = client
        .get(&url)
        .header("User-Agent", "github-cli-tool")
        .headers(headers)
        .send()
        .unwrap()
        .json::<Vec<Repository>>() // Ensure `reqwest` has "json" feature enabled
        .unwrap_or_default();

    let mut repos = response;
    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    repos.truncate(10);
    repos
}

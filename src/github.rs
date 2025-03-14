use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
}

pub fn fetch_repos(args: &crate::cli::Args) -> Vec<Repository> {
    let client = Client::new();

    let mut all_repos = vec![];
    for username in &args.usernames {
        let url = format!("https://api.github.com/users/{}/repos", username);
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(token) = &args.token {
            headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
        }

        let response = client
            .get(&url)
            .header("User-Agent", "github-cli-tool")
            .headers(headers)
            .send()
            .unwrap()
            .json::<Vec<Repository>>()
            .unwrap_or_default();

        all_repos.extend(response);
    }

    all_repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    all_repos.truncate(10);
    all_repos
}

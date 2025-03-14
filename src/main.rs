mod cli;
use clap::ArgMatches;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{fs, thread, time::Duration};

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: u32,
}

fn fetch_repos(username: &str, token: Option<&str>) -> Vec<Repository> {
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
        .json::<Vec<Repository>>()
        .unwrap_or_default();

    let mut repos = response;
    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    repos.truncate(10);
    repos
}

fn display_repos(repos: &[Repository]) {
    for repo in repos {
        println!(
            "\x1b[34mName:\x1b[0m {:<30} \x1b[33mStars:\x1b[0m {}",
            repo.name, repo.stargazers_count
        );
        println!("  \x1b[34mURL:\x1b[0m {}", repo.html_url);
        if let Some(desc) = &repo.description {
            println!("  \x1b[34mDescription:\x1b[0m {}", desc);
        }
        println!("---------------------------------------------");
    }
}

fn save_repos(repos: &[Repository], path: &str, format: &str) {
    let serialized_data = match format {
        "json" => serde_json::to_string_pretty(repos).unwrap(),
        "toml" => toml::to_string(repos).unwrap(),
        _ => panic!("Unsupported format"),
    };
    fs::write(path, serialized_data).expect("Failed to write file");
}

fn main() {
    let matches: ArgMatches = cli::parse_args();

    let usernames: Vec<String> = matches.get_many::<String>("username")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    
    let token = matches.get_one::<String>("token").cloned();
    let save_path = matches.get_one::<String>("save").cloned();

    let mut all_repos = vec![];
    for username in &usernames {
        thread::sleep(Duration::from_secs(5)); // Rate limit control (2 requests per 10 seconds)
        let repos = fetch_repos(username, token.as_deref());
        all_repos.extend(repos);
    }

    all_repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    all_repos.truncate(10);

    if let Some(path) = save_path {
        if path.ends_with(".json") {
            save_repos(&all_repos, &path, "json");
        } else if path.ends_with(".toml") {
            save_repos(&all_repos, &path, "toml");
        }
    } else {
        display_repos(&all_repos);
    }
}
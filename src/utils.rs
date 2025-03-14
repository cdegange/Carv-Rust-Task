use crate::github::Repository;
use std::fs;

pub fn display_repos(repos: &[Repository]) {
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

pub fn save_repos(repos: &[Repository], path: &str) {
    let serialized_data = if path.ends_with(".json") {
        serde_json::to_string_pretty(repos).unwrap()
    } else if path.ends_with(".toml") {
        toml::to_string(repos).unwrap()
    } else {
        panic!("Unsupported format");
    };

    fs::write(path, serialized_data).expect("Failed to write file");
}

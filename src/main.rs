mod cli;
mod github;
mod cache;
mod utils;

use crate::cli::parse_args;
use crate::github::fetch_repos;
use crate::utils::{display_repos, save_repos};

fn main() {
    if args = parse_args();

    let all_repos = fetch_repos(&args);

    if let Some(path) = &args.save_path {
        save_repos(&all_repos, path);
    } else {
        display_repos(&all_repos);
    }
}

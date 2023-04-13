/// Internal (standard)
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Mine
mod exitcode;
mod validation;

/// External (not mine)
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct GhRepoRes {
    name: String,
    clone_url: String,
}

async fn fetch_gh_repos(username: &str) -> Result<Vec<GhRepoRes>, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}/repos", username);

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("ACCEPT", "application/vnd.github+json")
        .header("User-Agent", username)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<Vec<GhRepoRes>>().await?),
        reqwest::StatusCode::BAD_REQUEST => {
            panic!("Bad request")
        }
        reqwest::StatusCode::NOT_FOUND => {
            panic!("GitHub user not found")
        }
        rest => panic!("GitHub response: {}", rest),
    }
}

// theme is a reference to a ColorfulTheme
fn prompt_username(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("GitHub username")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if validation::is_valid_username(input) {
                    Ok(())
                } else {
                    Err("Invalid username.")
                }
            }
        })
        .interact_text()
        .unwrap()
}

fn prompt_repos_to_clone(theme: &ColorfulTheme, repo_names: &Vec<String>) -> Vec<usize> {
    MultiSelect::with_theme(theme)
        .with_prompt("Pick repos to clone")
        .items(&repo_names[..])
        .interact()
        .unwrap()
}

async fn init(theme: &ColorfulTheme, repos: &Result<Vec<GhRepoRes>, reqwest::Error>) {
    let mut repo_map: HashMap<String, String> = HashMap::new();
    repo_map = match repos {
        Err(..) => panic!("dsads"),
        Ok(x) => x
            .iter()
            .map(|ss| (ss.name.clone(), ss.clone_url.clone()))
            .collect(),
    };

    let repo_names = repo_map.keys().cloned().collect::<Vec<String>>();

    let repos_to_clone = MultiSelect::with_theme(theme)
        .with_prompt("Pick repos to clone")
        .items(&repo_names[..])
        .interact()
        .unwrap();

    if repos_to_clone.is_empty() {
        panic!("No repos selected. Exiting.")
    }

    let clone_to_dir: String = Input::with_theme(theme)
        .with_prompt("Directory to clone to")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if validation::is_valid_directory(Path::new(input)) {
                    Ok(())
                } else {
                    Err("Invalid path.")
                }
            }
        })
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let proper_clone_to_dir = PathBuf::from(&clone_to_dir);

    for repo in repos_to_clone {
        let repo_name = &repo_names[repo];

        match repo_map.get(repo_name) {
            Some(clone_url) => {
                Command::new("git")
                    .args(["clone", clone_url])
                    .current_dir(&proper_clone_to_dir)
                    .spawn()
                    .expect("Failed to clone");
            }
            None => println!("Filler"),
        }
    }
}

#[tokio::main]
async fn main() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let username = prompt_username(&theme);
    let repos = fetch_gh_repos(&username).await;

    let repo_map: HashMap<String, String> = match repos {
        Err(..) => panic!("dsads"),
        Ok(x) => x
            .into_iter()
            .map(|repo| (repo.name, repo.clone_url))
            .collect(),
    };

    let repo_names = repo_map.keys().cloned().collect::<Vec<String>>();
    let repos_to_clone = prompt_repos_to_clone(&theme, &repo_names);

    std::process::exit(exitcode::OK)
}

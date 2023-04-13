/// Internal (standard)
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

/// Mine
mod exitcode;
mod git;
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

fn prompt_destination_dir(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
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
        .unwrap()
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
    let selected_repos = prompt_repos_to_clone(&theme, &repo_names);
    let destination_dir = prompt_destination_dir(&theme);
    let destination_dir_pathbuf = PathBuf::from(&destination_dir);

    for selected_repo in selected_repos {
        let repo_name = &repo_names[selected_repo];

        if let Some(clone_url) = repo_map.get(repo_name) {
            git::clone(&clone_url, &destination_dir_pathbuf)
        }
    }

    std::process::exit(exitcode::OK)
}

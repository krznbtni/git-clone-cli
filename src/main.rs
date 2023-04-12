use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn validate_username(s: &str) -> bool {
    // r: a raw string.
    // A raw string is just like a regular string,
    // except it does not process any escape sequences.
    // For example, "\\d" is the same expression as r"\d".
    let re = Regex::new(r"^[0-9A-Za-z_.-]+$").unwrap();
    re.is_match(s)
}

#[test]
fn validate_usernames() {
    let valid_username = "0valid_.-";
    let invalid_username = " invalid_.-/¤";
    assert_eq!(validate_username(valid_username), true);
    assert_eq!(validate_username(invalid_username), false);
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

async fn init() {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let username = Input::with_theme(&theme)
        .with_prompt("GitHub username")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                let valid = validate_username(input);

                if valid {
                    Ok(())
                } else {
                    Err("Invalid username.")
                }
            }
        })
        .interact_text()
        .unwrap();

    let mut repo_name_clone_url_map: HashMap<String, String> = HashMap::new();

    let repos = fetch_gh_repos(&username).await;
    match repos {
        Err(..) => panic!("dsads"),
        Ok(x) => {
            repo_name_clone_url_map = x
                .iter()
                .map(|ss| (ss.name.clone(), ss.clone_url.clone()))
                .collect();
        }
    }

    let repo_names = repo_name_clone_url_map
        .keys()
        .cloned()
        .collect::<Vec<String>>();

    let repos_to_clone = MultiSelect::with_theme(&theme)
        .with_prompt("Pick repos to clone")
        .items(&repo_names[..])
        .interact()
        .unwrap();

    if repos_to_clone.is_empty() {
        panic!("No repos selected. Exiting.")
    }

    // TODO 5: give input promp - enter directory to clone repos in
    let clone_to_dir: String = Input::with_theme(&theme)
        .with_prompt("Directory to clone to")
        .default(".".to_string())
        .interact_text()
        .unwrap();

    let proper_clone_to_dir = PathBuf::from(&clone_to_dir);

    for repo in repos_to_clone {
        let repo_name = &repo_names[repo];

        match repo_name_clone_url_map.get(repo_name) {
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
    init().await;
}

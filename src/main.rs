/// Internal (standard)
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

/// Mine
mod exitcode;
mod git;
mod github;
mod validation;

/// External (not mine)
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};

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
    let repos = github::fetch_user_repos(&username).await;

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

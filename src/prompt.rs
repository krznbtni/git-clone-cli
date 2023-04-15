use std::path::Path;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};

pub fn theme() -> ColorfulTheme {
    ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    }
}

pub fn username(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("GitHub username")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if super::validation::is_valid_username(input) {
                    Ok(())
                } else {
                    Err("Invalid username.")
                }
            }
        })
        .interact_text()
        .unwrap()
}

pub fn repos_to_clone(theme: &ColorfulTheme, repo_names: &Vec<String>) -> Vec<usize> {
    MultiSelect::with_theme(theme)
        .with_prompt("Pick repos to clone")
        .items(&repo_names[..])
        .interact()
        .unwrap()
}

pub fn destination_dir(theme: &ColorfulTheme) -> String {
    Input::with_theme(theme)
        .with_prompt("Directory to clone to")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if super::validation::is_valid_directory(Path::new(input)) {
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

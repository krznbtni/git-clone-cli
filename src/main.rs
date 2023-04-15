use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let theme = git_clone_cli::prompt::theme();
    let username = git_clone_cli::prompt::username(&theme);

    let repos = git_clone_cli::github::fetch_user_repos(&username).await;

    let repo_map: HashMap<String, String> = match repos {
        Err(..) => panic!("dsads"),
        Ok(x) => x
            .into_iter()
            .map(|repo| (repo.name, repo.clone_url))
            .collect(),
    };

    let repo_names = repo_map.keys().cloned().collect::<Vec<String>>();
    let selected_repos = git_clone_cli::prompt::repos_to_clone(&theme, &repo_names);
    let destination_dir = git_clone_cli::prompt::destination_dir(&theme);
    let destination_dir_pathbuf = PathBuf::from(&destination_dir);

    for selected_repo in selected_repos {
        let repo_name = &repo_names[selected_repo];

        if let Some(clone_url) = repo_map.get(repo_name) {
            git_clone_cli::git::clone(&clone_url, &destination_dir_pathbuf)
        }
    }

    std::process::exit(git_clone_cli::exitcode::OK)
}

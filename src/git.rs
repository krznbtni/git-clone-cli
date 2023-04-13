use std::path::PathBuf;
use std::process::Command;

pub fn clone(clone_url: &str, destination_dir: &PathBuf) {
    Command::new("git")
        .args(["clone", clone_url])
        .current_dir(destination_dir)
        .spawn()
        .expect("Error: failed to clone");
}

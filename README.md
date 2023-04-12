# Git Clone CLI

## How should the first version behave?

1. The User should be met with an input prompt, querying for a GitHub username.
2. The Program should call the GitHub API, querying for repositories available repositories.
3. The User should be met with a multiselect prompt and select which repositories to clone.
4. The User should be met with an input prompt, querying for a directory to clone the repositories to (default: current directory).

## How should future versions behave?

- Allow the User to select from where to clone repositories (GitHub, Gitlab, etc).
- Figure out how to fetch non-public repositories from for example GitHub (private, organizational).

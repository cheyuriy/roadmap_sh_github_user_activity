# GitHub User Activity CLI

This project is part of the [roadmap.sh GitHub User Activity project](https://roadmap.sh/projects/github-user-activity). It is a command-line tool to fetch and display recent GitHub events for a specified user.

## Features

- Fetches recent GitHub activity using the GitHub API.
- Displays event type, repository, and timestamp in the terminal.
- Handles errors gracefully, such as invalid usernames or API failures.

## Usage

1. Build the project using Cargo:
    ```sh
    cargo build
    ```

2. Run the CLI with a GitHub username:
    ```sh
    cargo run -- <username>
    ```
    or after installation
    ```sh
    github-cli-activity <username>
    ```

__Example:__
```sh
github-activity-cli dbt-labs
```

Output:
```sh
2025-04-23 11:39:11 UTC: 'PushEvent' event from login 'vladmdbt' in the 'dbt-labs/terraform-provider-dbtcloud'
2025-04-23 11:23:43 UTC: 'WatchEvent' event from login 'arnoldbagamba' in the 'dbt-labs/docs.getdbt.com'
2025-04-23 11:17:12 UTC: 'WatchEvent' event from login 'yussefabouahmed-ft' in the 'dbt-labs/dbt-jsonschema'
2025-04-23 11:05:55 UTC: 'WatchEvent' event from login 'Zhuravld' in the 'dbt-labs/dbt-mcp'
```

## Notes
The application uses the GitHub API endpoint: `https://api.github.com/users/<username>/events`.
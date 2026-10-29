# GitHub User Activity

A Rust command-line program that fetches a GitHub user's recent public activity and prints it as a readable timeline. It is a solution to the [GitHub User Activity](https://roadmap.sh/projects/github-user-activity) project from roadmap.sh.

## Overview

The program calls the GitHub REST API endpoint `https://api.github.com/users/<username>/events`, deserialises the returned events with `serde`, and prints one line per event, oldest first:

```
- 2025-10-09T12:34:56Z: Pushed 3 commits to owner/repo
```

## Features

Events are turned into messages as follows:

| Event type | Message |
| --- | --- |
| `PushEvent` | Pushed N commit(s) to the repository |
| `CreateEvent` | New repository created, or a new branch or tag created in a repository |
| `DeleteEvent` | Deleted a branch or tag from a repository |
| `ForkEvent` | Forked a repository, naming the new fork |
| `CommitCommentEvent` | Commented on a commit, with the short commit ID and the comment |
| `IssuesEvent` | Opened, closed or otherwise acted on an issue in a repository |
| `IssueCommentEvent` | Acted on a comment on an issue, with the comment text |
| `GollumEvent` | Created or updated N wiki page(s) |
| `MemberEvent` | Added or otherwise changed a collaborator on a repository |
| `PublicEvent` | Made a repository public |

Pull request, review, release, sponsorship and watch (star) events are recognised but print an empty message for now.

## Tech stack

- Rust, 2024 edition
- [`reqwest`](https://crates.io/crates/reqwest) (blocking client) for HTTP
- [`serde`](https://crates.io/crates/serde) and [`serde_json`](https://crates.io/crates/serde_json) for JSON

## Getting started

Requires a Rust toolchain that supports the 2024 edition (Rust 1.85 or later) and network access to `api.github.com`.

```sh
cargo build --release
cargo run --release
```

## Usage

The username is not yet read from the command line: it is hard-coded in `main` in `src/main.rs` (`build_url("0atman")`). To see another user's activity, change that string and run `cargo run` again.

Requests are unauthenticated, so they are subject to GitHub's rate limit for anonymous API calls.

## Project structure

```
src/
├── main.rs    # HTTP request, event-to-message formatting and output
└── event.rs   # Event, Repo and EventType types deserialised from the API
```

## Status and limitations

This is a work in progress against the roadmap.sh brief, which asks for the username as a command-line argument. Still to do:

- take the username as an argument instead of hard-coding it
- write messages for the pull request, review, release, sponsorship and watch event types
- handle errors gracefully: the program panics on an API error response (such as an unknown user or a rate-limit response), on an event type not in its list, or when an expected payload field is missing
- the issue comment message prints the whole issue object as JSON rather than its title or number

## Acknowledgements

Project brief from [roadmap.sh](https://roadmap.sh/projects/github-user-activity).

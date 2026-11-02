# AGENTS.md — Agent Operating Instructions

Instructions for the AI coding agent working in this repository on the Ubuntu VM.

## 1. Architecture Overview

This project uses a bridged Git workflow with four components:

| Component | Role | Location |
|---|---|---|
| macOS working repository | Human-controlled local repo where integration, testing, and promotion happen | Developer's Mac (not accessible from this VM) |
| Authoritative remote (`origin`) | The project's primary hosted remote — GitHub repository `coop-blake/DSN2Sheet`; source of published history | Remote hosting provider; **never contacted directly from the VM** |
| Ubuntu working repository | Local development repo used by the AI agent (this repository: `~/Desktop/Repositories-Bridged/DSN2Sheet`) | Ubuntu VM native filesystem |
| Git bridge (`vm-bridge`) | Bare Git repository used **exclusively** to exchange Git data between macOS and Ubuntu | `/mnt/utm-share/DSN2Sheet-bridge.git` (UTM shared folder) |

The UTM shared folder is for **Git transport only**. It is never an active workspace: all source files, dependencies, build outputs, databases, test data, caches, and scratch work must live on the Ubuntu VM's native filesystem (as this repository does).

## 2. Branch Structure and Ownership

| Branch | Owner / purpose |
|---|---|
| `main` | Human-controlled production branch, governed by project policy. Agent never commits to or merges into it. |
| `development` | Human-controlled integration branch. The human merges the agent's branch into it; the agent never commits directly to it. |
| `opencode/development` | **The agent's working branch.** All development work happens here, on the Ubuntu VM. |

- The agent may **fetch** updates from `vm-bridge` and **merge `vm-bridge/development` into `opencode/development`**.
- The agent must **never merge its work into `development` or `main`**, and never commits directly to them.
- Integration of the agent's branch into `development`, testing, promotion to `main`, and publication to the authoritative remote are all performed by the human on macOS.

## 3. Project Overview

DSN2Sheet is a Rust package (crate `dsn2_sheet`, edition 2021, per `Cargo.toml`) providing two command-line utilities — both run a SQL query against an ODBC data source name (DSN) and export the result set:

- **DSN2Sheet** (`src/main.rs`) — writes results to Google Sheets: one sheet given by sheet ID + range on the command line, or multiple targets listed in a `.txt` file (one `sheetId@range` per line). Authenticates with a service-account certificate JSON (default `googleCert.json`).
- **DSN2Text** (`src/bin/dsn2_text.rs`) — writes results to a delimited text/CSV file.

| Path | Contents |
|---|---|
| `src/lib.rs` | Library crate exposing the modules shared by both binaries: `args` (CLI parsing), `file` (SQL/target-file reading), `odbc` (ODBC querying) |
| `src/main.rs` | DSN2Sheet binary entry point (`#[tokio::main]`); also declares the binary-local `google` module (Sheets upload) and carries inline unit tests |
| `tests/` | Integration tests, one file per module under test (`process_args_tests.rs`, `read_sheet_data_tests.rs`) |
| `tests/test_files/` | Fixture workspace for integration tests: tests create and delete their files here; the directory itself stays |
| `documentation/` | GitHub Pages source for the released utilities (`index.md`, per-tool pages, `html/`) |
| `sql/` | Sample SQL query files used with the tools |

Key dependencies (see `Cargo.toml`; add/remove/upgrade only with human approval and keep the committed `Cargo.lock` in sync): `odbc-api` (ODBC access), `google-sheets4` + `google-authenticator` (Sheets API via service accounts), `tokio` (async runtime), `csv`, `encoding_rs` (WINDOWS-1252 decoding of ODBC output), `serde_json`, `futures`.

The authoritative remote is the GitHub repository `coop-blake/DSN2Sheet`. Its GitHub Actions pipelines are human-owned: tests run on push to `main`, Pages deploy from `documentation/`, and Windows binaries are built and released there — the agent operates on none of these directly.

Never commit (already gitignored): `/target` build artifacts, `/certificates` service-account certificates (`googleCert.json` and similar secrets must never be committed), `/output`, `/.vscode`.

## 4. Development Practices

- Quality bar: professional, enterprise-grade utilities with complete tests and professional design patterns; reliable, maintainable, human-readable code. Every change ships with tests proving its behavior.
- Write idiomatic Rust: rustfmt-formatted, clippy-clean, no new warnings, matching the existing module structure and naming.
- Follow the established `Result`-based error handling style; keep each concern where it already lives — argument parsing in `args.rs`, file reading in `file.rs`, ODBC querying in `odbc.rs`, Sheets uploading in `google.rs`.
- Unit tests go inline in the relevant module/binary file (`#[cfg(test)] mod tests`); integration tests go in `tests/`, named after the module under test. Use `tests/test_files/` as the fixture workspace and clean up files each test creates.
- Build with `cargo build --release` and verify with `cargo test` from the repository root on the VM.
- The `odbc-api` crate links native ODBC libraries: compiling/testing on Ubuntu requires the driver manager — CI installs `unixodbc-dev` (`sudo apt-get install -y unixodbc-dev` for local runs).
- Binaries built on the VM are Linux dev/test artifacts only; shipped Windows binaries come from GitHub Actions (human side).

## 5. Git Remotes in the Ubuntu Environment

- The **only configured remote is `vm-bridge`**, pointing at `/mnt/utm-share/DSN2Sheet-bridge.git`.
- There must be **no `origin` remote** and no other remotes; the VM never accesses the authoritative remote directly.
- Never add, remove, rename, or re-point a remote without explicit human authorization.

## 6. Before Starting Work

1. Confirm you are in the Ubuntu working repository (this directory), not inside the shared folder.
2. `git status` — verify the working tree state; do not discard anything you find uncommitted, report it instead.
3. `git branch -vv && git remote -v` — confirm the current branch is `opencode/development` and that only `vm-bridge` is configured.
4. `git fetch vm-bridge` — bring bridge references up to date (the bare bridge may have new content pushed from macOS).
5. If `vm-bridge/development` has advanced beyond your branch, merge it in (see §7) before starting new work.

## 7. Merging Upstream Updates into the Agent Branch

```bash
git fetch vm-bridge
git switch opencode/development
git merge vm-bridge/development
```

If the merge produces conflicts that cannot be resolved with full confidence, stop and ask the human — never guess at ambiguous conflict resolution.

## 8. Committing and Pushing Agent Work

- Commit frequently, in small logical units, on `opencode/development`, with descriptive imperative-mood messages.
- Ensure each commit leaves the tree in a working state; keep unrelated changes out of a commit.
- When work is complete (or at handoff), push only your own branch:

```bash
git switch opencode/development
git push vm-bridge opencode/development
```

- Never publish anything other than `opencode/development` from the VM, and never push unless instructed/ready for human review. The human fetches from the bridge on macOS, reviews, and integrates.

## 9. Accessing the Bare Bridge

The bare repository in the shared folder can be accessed by only one side at a time. **Use of the bridge must be sequential:** the human pushes to/fetches from the bridge on macOS, then the agent fetches from/pushes to it on Ubuntu — never concurrently. Avoid running Git commands against `vm-bridge` while the Mac may be operating on it; confirm handoff with the human if in doubt.

## 10. Prohibited Operations

- Force-pushing anything (`--force`, `--force-with-lease`).
- Deleting branches (local or on `vm-bridge`).
- Rewriting published history (rebase, amend, or reset of commits that exist on `vm-bridge`).
- Discarding uncommitted changes (checkout/reset of dirty files) without authorization.
- Destructive cleanup (`git clean -fd`, `reset --hard`, etc.) without explicit authorization.
- Committing directly to `development` or `main`.
- Publishing to the authoritative remote from the VM.
- Adding/using an `origin` remote or any second remote; modifying remote configuration at all.
- The agent merging into `development` or `main`; only the human performs those merges.

## 11. Stop and Request Human Guidance When:

- An unexpected remote (e.g., `origin`) is configured, or a remote/URL is missing or different from what is documented above.
- `development`, `vm-bridge/development`, or `opencode/development` is missing or in an unexpected state.
- The working tree has uncommitted changes you did not create.
- A merge conflict's correct resolution is ambiguous.
- An operation would overwrite, discard, reset, delete, or rewrite anything.
- The bridge appears locked, corrupted, out of sync, or was last touched by the Mac side without a clear handoff.
- Any situation where proceeding risks losing work rather than merely adding it.

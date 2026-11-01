# AGENTS.md — Agent Operating Instructions

Instructions for the AI coding agent working in this repository on the Ubuntu VM.

## 1. Architecture Overview

This project uses a bridged Git workflow with four components:

| Component | Role | Location |
|---|---|---|
| macOS working repository | Human-controlled local repo where integration, testing, and promotion happen | Developer's Mac (not accessible from this VM) |
| Authoritative remote (`origin`) | The project's primary hosted remote (GitHub/GitLab/Gitea/etc.); source of published history | Remote hosting provider; **never contacted directly from the VM** |
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

## 3. Git Remotes in the Ubuntu Environment

- The **only configured remote is `vm-bridge`**, pointing at `/mnt/utm-share/DSN2Sheet-bridge.git`.
- There must be **no `origin` remote** and no other remotes; the VM never accesses the authoritative remote directly.
- Never add, remove, rename, or re-point a remote without explicit human authorization.

## 4. Before Starting Work

1. Confirm you are in the Ubuntu working repository (this directory), not inside the shared folder.
2. `git status` — verify the working tree state; do not discard anything you find uncommitted, report it instead.
3. `git branch -vv && git remote -v` — confirm the current branch is `opencode/development` and that only `vm-bridge` is configured.
4. `git fetch vm-bridge` — bring bridge references up to date (the bare bridge may have new content pushed from macOS).
5. If `vm-bridge/development` has advanced beyond your branch, merge it in (see §5) before starting new work.

## 5. Merging Upstream Updates into the Agent Branch

```bash
git fetch vm-bridge
git switch opencode/development
git merge vm-bridge/development
```

If the merge produces conflicts that cannot be resolved with full confidence, stop and ask the human — never guess at ambiguous conflict resolution.

## 6. Committing and Pushing Agent Work

- Commit frequently, in small logical units, on `opencode/development`, with descriptive imperative-mood messages.
- Ensure each commit leaves the tree in a working state; keep unrelated changes out of a commit.
- When work is complete (or at handoff), push only your own branch:

```bash
git switch opencode/development
git push vm-bridge opencode/development
```

- Never publish anything other than `opencode/development` from the VM, and never push unless instructed/ready for human review. The human fetches from the bridge on macOS, reviews, and integrates.

## 7. Accessing the Bare Bridge

The bare repository in the shared folder can be accessed by only one side at a time. **Use of the bridge must be sequential:** the human pushes to/fetches from the bridge on macOS, then the agent fetches from/pushes to it on Ubuntu — never concurrently. Avoid running Git commands against `vm-bridge` while the Mac may be operating on it; confirm handoff with the human if in doubt.

## 8. Prohibited Operations

- Force-pushing anything (`--force`, `--force-with-lease`).
- Deleting branches (local or on `vm-bridge`).
- Rewriting published history (rebase, amend, or reset of commits that exist on `vm-bridge`).
- Discarding uncommitted changes (checkout/reset of dirty files) without authorization.
- Destructive cleanup (`git clean -fd`, `reset --hard`, etc.) without explicit authorization.
- Committing directly to `development` or `main`.
- Publishing to the authoritative remote from the VM.
- Adding/using an `origin` remote or any second remote; modifying remote configuration at all.
- The agent merging into `development` or `main`; only the human performs those merges.

## 9. Stop and Request Human Guidance When:

- An unexpected remote (e.g., `origin`) is configured, or a remote/URL is missing or different from what is documented above.
- `development`, `vm-bridge/development`, or `opencode/development` is missing or in an unexpected state.
- The working tree has uncommitted changes you did not create.
- A merge conflict's correct resolution is ambiguous.
- An operation would overwrite, discard, reset, delete, or rewrite anything.
- The bridge appears locked, corrupted, out of sync, or was last touched by the Mac side without a clear handoff.
- Any situation where proceeding risks losing work rather than merely adding it.

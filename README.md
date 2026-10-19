## It's Gitu! - A Git porcelain *outside* of Emacs
[![CI](https://github.com/altsem/gitu/actions/workflows/ci.yml/badge.svg)](https://github.com/altsem/gitu/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/altsem/gitu/graph/badge.svg?token=5YWPU7GWFW)](https://codecov.io/gh/altsem/gitu)

A terminal user interface for Git. Inspired by Magit.

<img style="width: 720px" src="vhs/rec.gif"/>

### Features
Gitu aims to implement many of the core features of Magit over time.
It should be familiar to any previous Magit users.\
Here's a list of so-far supported features:
- **Staging/Unstaging** _(file, hunk, line)_ 
- **Showing** _(view commits / open EDITOR at line)_
- **Branching** _(checkout, checkout new)_
- **Committing** _(commit, amend, fixup)_
- **Fetching**
- **Logging** _(current, other)_
- **Pulling / Pushing** _to/from configured upstream/pushDefault_
- **Rebasing** _(elsewhere, abort, continue, autosquash, interactive)_
- **Resetting** _(soft, mixed, hard)_
- **Reverting** _(commit)_
- **Stashing** _(save, pop, apply, drop)_

### Keybinds
Keybinds try mimic Magit, while staying Vim-like.
Press `?` to open help. Set `general.always_show_help.enabled = true` to keep help visible.


<img style="width: 720px" src="vhs/help.png"/>

### Fork defaults
This fork selects individual changed lines with Vim keys.

| Keys | Action |
| --- | --- |
| `j` / `↓` | Select the next changed line or header. |
| `k` / `↑` | Select the previous changed line or header. |
| `h` / `←` | Navigate left through sections. |
| `l` / `→` | Navigate right through sections. |
| `Ctrl+j` / `Ctrl+↓` | Move down without selecting diff lines. |
| `Ctrl+k` / `Ctrl+↑` | Move up without selecting diff lines. |
| `Alt+j` / `Alt+k` | Select the next or previous section. |
| `Tab` | Fold or unfold the selected section. |
| `?` | Open help. |
| `L` | Open the log menu. |

```text
Section navigation
 ├── h / ←
 │    ├── Expanded section ──► EXECUTE: Fold section
 │    ├── Parent section exists ──► EXECUTE: Select parent section
 │    └── Otherwise ──► EXECUTE: Keep selection
 └── l / →
      ├── Collapsed section ──► EXECUTE: Unfold section
      ├── Expanded section with selectable children ──► EXECUTE: Select first selectable child
      └── Otherwise ──► EXECUTE: Keep selection
```

Unchanged context lines remain unselectable.

Install this fork from the checkout:
```shell
cargo install --path . --locked --force
```

### Configuration
The environment variables `VISUAL`, `EDITOR` or `GIT_EDITOR` (checked in this order) dictate which editor Gitu will open. This means that e. g. commit messages will be opened in the `GIT_EDITOR` by Git, but if the user wishes to do edits to the actual files in a different editor, `VISUAL` or `EDITOR` can be set accordingly.

Configuration is also loaded from:
- Linux:   `~/.config/gitu/config.toml`
- macOS:   `~/.config/gitu/config.toml`
- Windows: `%USERPROFILE%\AppData\Roaming\gitu\config.toml`

, refer to the [default configuration](src/default_config.toml).

### Installing Gitu
Follow the install instructions: [Installing Gitu](docs/installing.md)\
Or install from your package manager:

[![Packaging status](https://repology.org/badge/vertical-allrepos/gitu.svg)](https://repology.org/project/gitu/versions)

Shell completions can be generated with `gitu completion <shell>`, see [Shell completions](docs/installing.md#shell-completions).

### Contributing
While a part of me do with issues/PRs could be closed a lot sooner to respect the work of contributors, I've also coded/tested/reviewed this myself on my spare time. And therefore it needs to stay interesting for myself and not become a chore. There are a few rough edges and missing features in Gitu, it'll never become a full magit-clone. While I'm not against opening up pull-requests, I just wanted to be clear on the expectations.
You are always free to fork/extend this project to your heart's desire.

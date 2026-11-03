# wt

Git worktree resolver for the `~/.worktrees/<path-key>/<branch>` convention: it finds the worktree registered for a branch, creates it when there is none, and links overlay files from the main worktree into it.

## Install

- Install with [mise](https://mise.jdx.dev): `mise use -g github:phatblat/wt`.
- Or download `wt-<version>-<target>.tar.gz` (`aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, or `x86_64-unknown-linux-gnu`) from https://github.com/phatblat/wt/releases, check it against `SHA256SUMS`, and put `wt` on `PATH`.
- Or build from source: `cargo install --git https://github.com/phatblat/wt --locked`.

## Usage

```text
usage: wt [<branch>] | wt <action> [<branch>] [flags]

actions:
  path <branch>       print the path that would be used; mutates nothing
  resolve <branch>    print the registered worktree path, or exit 3
  switch <branch>     resolve, else create; print the path
  create <branch>     create only; exit 4 if already registered
  remove <branch>     git worktree remove + prune; never touches the branch
  list                one <path>\t<branch> line per worktree
  verify <branch>     HOME-remapped `just check` (implies --allow-home)
  shell <branch>      HOME-remapped interactive $WT_SHELL, default `zsh -i` (implies --allow-home)
  overlay [<branch>]  apply this repo's link manifest to the worktree

flags, accepted in any position:
  --allow-home        opt in to worktrees for the repo at $HOME
  --repo <dir>        derive the repo from <dir> instead of the cwd
  --force             remove: pass --force to `git worktree remove`
  --cd-file <path>    switch: also write the resolved directory to <path>
  -h, --help          print this text
  -V, --version       print the version

With no action and no branch, wt runs an fzf picker over the repo's worktrees.
The first positional is an action only when it is exactly one of the nine verbs
above; anything else is a branch and the action is `switch`. A branch named
`list` must be written `wt switch list`.
```

## Worktree location

- For the repository at `$HOME` (needs `--allow-home`; `verify` and `shell` imply it), worktrees go under `${DOTFILES_WT_ROOT:-~/.worktrees/dotfiles}/<branch>`, and its path-key is `dotfiles`.
- For any other repository, worktrees go under `~/.worktrees/<path-key>/<branch>`. The path-key is the repository root with the leading `$HOME/` removed and every `/` replaced by `-`; for example, `~/dev/rust/wt` becomes `dev-rust-wt`.
- A slashed branch nests (`feat/topic` → `…/feat/topic`).

## Shell integration

A process cannot change its parent shell's directory, so `wt` writes the directory to enter to the file named by `--cd-file`, and a shell function `cd`s there. For zsh:

```zsh
wt() {
  local cd_file rc dest
  cd_file=$(mktemp "${TMPDIR:-/tmp}/wt-cd.XXXXXX") || return $?
  command wt --cd-file "$cd_file" "$@"
  rc=$?
  dest=$(< "$cd_file")
  rm -f -- "$cd_file"
  [[ -n "$dest" ]] && cd "$dest"
  return "$rc"
}
```

The wrappers used day to day are [zsh](https://github.com/phatblat/dotfiles/blob/main/.config/zsh/functions/wt) and [nushell](https://github.com/phatblat/dotfiles/blob/main/.config/nushell/autoload/wt.nu).

## Exit codes

- `0`: success.
- `1`: a git or exec failure. `verify` returns `just check`'s exit code, and a failed `git worktree add` returns git's.
- `2`: usage error, unknown action, or `fzf` missing for the picker.
- `3`: no worktree registered for the branch.
- `4`: refused. Causes: `create` on an already-registered branch, a path that is occupied or registered under a mismatched directory, `git fetch` failure, the repository at `$HOME` without `--allow-home`, `remove` of the main worktree or of the worktree you are in, or an overlay destination that exists and is not a symlink.

## Environment

| Variable | Meaning | Default |
| --- | --- | --- |
| `WT_SHELL` | The program and arguments `wt shell` execs, split on whitespace. Unset or blank means the default. | `zsh -i` |
| `WT_OVERLAY_DIR` | The overlay manifest directory. | `~/.config/wt/overlay` |
| `DOTFILES_WT_ROOT` | The worktree root for the repository at `$HOME`. | `~/.worktrees/dotfiles` |

## Overlay manifests

- `switch` (on create and on resolve) and `overlay` symlink entries from the main worktree into the target worktree.
- The manifest is `$WT_OVERLAY_DIR/<path-key>`, falling back to `$WT_OVERLAY_DIR/default`. With neither, nothing is linked.
- Format: one worktree-relative path per line. A line starting with `#` is a comment, blank lines are ignored, and surrounding whitespace is trimmed.
- An entry missing from the main worktree, or already a symlink in the target, is skipped. An entry that exists in the target and is not a symlink stops with exit 4.
- Targeting the main worktree links nothing.

## Development

Run `just deps` to install the pinned toolchain, then `just check`, which runs `cargo test --locked`. The tests need `git` and `zsh` on `PATH`.

## Releasing

Set `version` in `Cargo.toml`, run `cargo update --workspace`, commit, then push tag `v<version>`. The release workflow builds every target and publishes `wt-<version>-<target>.tar.gz` and `SHA256SUMS`.

## License

[MIT](LICENSE.md) © Ben Chatelain

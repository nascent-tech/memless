<!-- A maintainer squash-merges this pull request: its title (or, for a single
     commit, that commit's title) becomes the commit on main. Write both as a
     Conventional Commit, such as fix(php): … or feat(node): … -->

## Why

<!-- The problem this solves. Link the issue: Closes #… -->

## What changes

<!-- What the change does, bridge by bridge when it touches them. -->

## Checks

- [ ] `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo build --release -p memless-capi`
- [ ] Bridge tests: PHP (`vendor/bin/phpunit`), Go (`go test ./...`), Node.js (`npm test`, `npm run typecheck`)
- [ ] `bash harness/parity/run.sh`, with a new parity case if the behaviour changed
- [ ] The three bridges changed together, if a bridge surface changed
- [ ] `README.md`, the bridge READMEs and `CHANGELOG.md` (`## [Non publié]`) updated where a user would notice

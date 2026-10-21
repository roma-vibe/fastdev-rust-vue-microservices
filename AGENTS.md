# Rust + Vue microservices skeleton — instructions for AI agents

This repository is a fastDev skeleton (`rust-vue-microservices`). It is not a project: fastDev copies `files/` into
new projects and renders `*.tmpl` files. Read the fastDev skeleton authoring guide first
(MCP tool `get_authoring_guide`, or `docs/skeleton-authoring.md` in the fastDev repository).

- Change `template.toml` and `files/` only; `files/AGENTS.md.tmpl` and `files/SPEC.md.tmpl` are the
  instructions of the future projects, not of this repository.
- Do not commit, tag or edit `CHANGELOG.md` by hand: validate, verify and publish through fastDev
  (`validate_skeleton`, `verify_skeleton`, `publish_skeleton`). Publishing commits, creates the
  `vX.Y.Z` tag, pushes and updates the registry.
- Never move or delete a published tag.
- Everything is written in English.
- The `auth` feature switches files by `features.auth` lists and `@auth=on|off` alternatives
  (`crates/core/src/lib.rs`, `caller.rs`, `Cargo.lock`). `[verify] variants` cannot turn features
  off, so before publishing also create a project with `auth` off (`create_project`, version
  `draft`) and run `npm run check` and `npm run build` in it.
- Two `Cargo.lock` files: regenerate both (`cargo generate-lockfile` in a project with auth on and
  one with auth off) whenever Rust dependencies change.

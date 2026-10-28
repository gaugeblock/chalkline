# Changelog

All notable changes to `chalkline` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Multi-repository workspace auditing via `chalkline docs check --workspace` / `--all` (GB-103).
- Semantic AST drift policies: boundary & wire type contract auditing against `SPEC.md`, Clap CLI command/flag checks against `README.md`, mandatory `AGENTS.md` operational laws checks, and link integrity (GB-104).
- Native Markdown AST heading section targeting via `pulldown-cmark` in `chalkline docs sync` (zero HTML comments) and production scaffolding templates (GB-105).
- Comprehensive elevation of all 9 canonical documents across `chalkline` to gold standard (GB-119).

## [v0.1.0-beta.3] - YYYY-MM-DD

### Added
- Initialize `reasoning_effort` in `chalkline`'s `build_planner_request` to support planner request construction. (2181c1b)
- Initialize the local backlog store with `CH` prefix for scoped task tracking. (78cf78f)

### Fixed
- No fixes in this release.

### Changed
- Migrate scoped tasks and design plans into the updated `chalkline` backlog/design structure. (c6f55e3)

### Maintenance
- Standardize the `chalkline` README and establish the 9-document architecture. (5cfbfe6)
- Update `chalkline` release history with alpha and beta milestones. (3535f6d)

### Documentation
- Standardize README to 9-section template and establish AST-grounded 9-document architecture

## [0.1.0-beta.2] - 2026-10-24

### Added
- `chalkline-core`: 9-document repository architecture manager (`.gaugeblock/docs.toml`) with orthogonal boundary enforcement (GB-18).
- `chalkline-core`: AST drift inspector, brownfield markdown disentangler, and migration plan compiler (GB-18).
- `chalkline-cli`: `chalkline docs` command suite (`audit`, `init`, `plan`, `apply`) for 9-doc lifecycle (GB-18).
- `chalkline-cli`: upgrade interactive inquirer prompts to `inquire 0.9`.

## [0.1.0-beta.1] - 2026-10-22

### Added
- `chalkline-core`: interactive specification planner and `/grill-me` ambiguity resolution engine.
- `chalkline-core`: autonomous repository reconnaissance engine querying `miter-engine` for AST patterns and conventions.
- `chalkline-core`: `SharedTypeContract` compiler for polyglot wire boundaries.
- `chalkline-core`: topological DAG compiler generating verified `TaskNode` dependency graphs into `storypole`.
- `chalkline-cli`: interactive terminal wizard (`chalkline grill`), `chalkline compile-dag`, `chalkline sync`, `chalkline recon`.
- `chalkline-mcp`: stdio JSON-RPC 2.0 Model Context Protocol server exposing `chalkline_recon`, `chalkline_grill`, `chalkline_compile_dag`, `chalkline_sync`.

## [0.0.1-alpha.1] - 2026-10-21

### Added
- Initial namespace reservation on crates.io and GitHub repository scaffold.


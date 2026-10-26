# Changelog

All notable changes to `chalkline` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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


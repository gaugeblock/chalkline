# `chalkline` — Specification Planner, DAG Compiler & 9-Doc Architecture Manager

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** deterministic neurosymbolic suite.

> In carpentry, a **chalk line** is snapped taut across rough lumber to establish an unwavering, straight reference guide *before* any cuts are made. In the `gaugeblock` ecosystem, `chalkline` snaps the architectural reference line before code generation begins: it reconnoiters existing conventions, grills specification ambiguities, locks cross-language wire contracts, compiles acyclic task DAGs into `storypole`, and enforces the canonical 9-document repository architecture.

---

## 1. Architecture & Capabilities

```text
┌────────────────────────────────────────────────────────┐
│                   Target Codebase & Spec               │
│         SPEC.md  •  Cargo.toml  •  .gaugeblock/        │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ chalkline Core Engine                                  │
│ - Recon: miter-engine AST inspection                   │
│ - Grill: DecisionHole interactive resolver             │
│ - DAG Compiler: Topological task sequencing            │
│ - Docs Manager: 9-document architecture & AST audits   │
└───────────────┬────────────────────────┬───────────────┘
                │                        │
                ▼                        ▼
┌───────────────────────────┐ ┌──────────────────────────┐
│ CLI (`chalkline`)         │ │ MCP Stdio Server         │
│ - chalkline grill         │ │ - chalkline_recon        │
│ - chalkline compile-dag   │ │ - chalkline_grill        │
│ - chalkline docs check    │ │ - chalkline_compile_dag  │
│ - chalkline docs scaffold │ │ - chalkline_sync         │
└───────────────────────────┘ └──────────────────────────┘
```

1. **Reconnaissance (`recon`):** Queries `miter-engine` and repository manifests to discover existing conventions (error crates, async runtimes, testing harnesses) so agents never ask questions the code already answers.
2. **Ambiguity Griller (`grill`):** Identifies true `DecisionHole` architectural forks, presents tradeoff summaries with impacted symbols, and prompts users via interactive terminal TUI (`inquire`) or stdio MCP.
3. **Cross-Language Contracts (`contracts_gen`):** Locks neutral `SharedTypeContract` schemas and exports JSON schemas for downstream `dado` polyglot code generation.
4. **Topological DAG Compiler (`compile-dag`):** Compiles approved spec tasks into sequenced, acyclic task nodes and dependencies directly within `storypole` (`.storypole/ledger.jsonl`).
5. **9-Document Architecture Manager (`docs`):** Audits and scaffolds the canonical 9-document taxonomy, verifying markdown claims against code AST, Clap argument definitions, and Git state.

---

## 2. Workspace Crate Layout

* **`chalkline`**: Root facade re-exporting `chalkline-core`.
* **`crates/chalkline-core`**: Pure domain types, Markdown AST spec parser, recon engine, ambiguity griller, 9-doc architecture manager, and storypole DAG compiler.
* **`crates/chalkline-cli`**: Clap binary entrypoint (`chalkline`).
* **`crates/chalkline-mcp`**: JSON-RPC 2.0 stdio Model Context Protocol (MCP) server.

---

## 3. Installation & CLI Quickstart

```bash
# Install globally via cargo
cargo install --path crates/chalkline-cli

# 1. Interactive /grill-me session resolving decision holes
chalkline grill "Add persistent API rate limiting"
chalkline grill --compile "Add OAuth2 PKCE authentication"

# 2. Codebase reconnaissance
chalkline recon
chalkline recon --json

# 3. Compile markdown spec to Storypole DAG
chalkline compile-dag --spec SPEC.md

# 4. Sync task checkboxes with Storypole completion
chalkline sync --spec SPEC.md

# 5. Audit 9-document repository architecture against code AST
chalkline docs check
chalkline docs check -d path/to/repo

# 6. Scaffold missing canonical documents
chalkline docs scaffold
```

---

## 4. Model Context Protocol (MCP) Reference

| Tool | Parameters | Description |
|---|---|---|
| `chalkline_recon` | `target_dir?` | Query `miter-engine` for detected language conventions, error handlers, and async runtimes. |
| `chalkline_grill` | `prompt, spec_path?` | Execute non-interactive ambiguity analysis and extract decision holes. |
| `chalkline_compile_dag` | `spec_path, auto_link?` | Parse Markdown specification tasks and compile topological nodes into Storypole. |
| `chalkline_sync` | `spec_path` | Reconcile markdown task checklists with current Storypole ledger states. |

---

## 5. Repository Documentation

`chalkline` adheres strictly to the canonical **9-Document Architecture**:
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Local toolchain prerequisites, contributor etiquette, and test execution.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Internal sub-crate topologies, spec parsing pipeline, and doc engine design.
* **[`SPEC.md`](./SPEC.md):** Behavioral invariants, wire schemas, DecisionHole AST types, and error codes.
* **[`AGENTS.md`](./AGENTS.md):** Autonomous systems directives, <=250 LOC constraints, and zero-unwrap rules.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Historical release entries and unreleased changes.

---

## 6. Verification & Development Directives

* **File Size Ceiling:** All `.rs` files must strictly remain under 250 LOC.
* **Tiered Verification:**
  * Tier 1: `cargo check -p chalkline-core` (<2s).
  * Tier 2: `cargo fmt --check -p chalkline-core && cargo clippy -p chalkline-core && cargo test -p chalkline-core`.
  * Tier 3: `chalkline docs check -d .` before task verification.

---

## 7. License

Licensed under the [Apache License, Version 2.0](LICENSE).
Part of the [`gaugeblock`](https://gaugeblock.dev) suite.

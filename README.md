# `chalkline` — Specification Planner, DAG Compiler & 9-Doc Architecture Manager

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** deterministic neurosymbolic suite.

> In carpentry, a **chalk line** is snapped taut across rough lumber to establish an unwavering, straight reference guide *before* any cuts are made. In the `gaugeblock` ecosystem, `chalkline` snaps the architectural reference line before code generation begins: it reconnoiters existing conventions, grills specification ambiguities, locks cross-language wire contracts, compiles acyclic task DAGs into `storypole`, and enforces the canonical 9-document repository architecture.

---

## Why `chalkline`?

| Without `chalkline` | With `chalkline` |
|---|---|
| **Hallucinated Conventions:** Agents guess libraries or ask repetitive questions about logging and error crates already in the repo. | `chalkline recon` queries `miter-engine` AST in <50ms and injects existing runtime and error conventions automatically. |
| **Silent Ambiguity Assumptions:** Agents make silent, arbitrary decisions on critical database, schema, or API trade-offs. | `chalkline grill` extracts formal `DecisionHole` forks and prompts humans via interactive terminal TUI or stdio MCP. |
| **Linear, Fragile Todo Lists:** Markdown task lists contain circular dependencies and lack validation. | `chalkline compile-dag` compiles specification tasks into verified, acyclic topological task DAGs in `storypole`. |
| **Documentation Drift:** Architecture docs and READMEs decay silently as code AST and CLI commands evolve. | `chalkline docs check --workspace` audits 9 canonical documents against code AST, and `chalkline docs sync` keeps sections in lockstep. |

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
chalkline docs check --workspace

# 6. Synchronize AST-grounded sections into canonical documents
chalkline docs sync
chalkline docs sync --apply

# 7. Scaffold missing canonical documents
chalkline docs scaffold

# 8. Disentangle a brownfield document into 9 canonical documents
chalkline docs migrate legacy-README.md --apply
```

---

## 4. CLI Reference

| Command | Description |
|---|---|
| `chalkline recon` | Codebase reconnaissance using `miter-engine` |
| `chalkline grill` | Conduct interactive or non-interactive ambiguity resolution session |
| `chalkline compile-dag` | Compile tasks from markdown spec into a Storypole DAG |
| `chalkline sync` | Synchronize spec checkboxes with Storypole task status |
| `chalkline docs check` | Audit 9-document architecture against code AST and policies |
| `chalkline docs sync` | Synchronize AST-grounded sections into canonical documents |
| `chalkline docs scaffold` | Scaffold missing canonical documents using AST-grounded templates |
| `chalkline docs migrate` | Disentangle brownfield documents into the 9 canonical documents |
| `chalkline mcp` | Launch the stdio JSON-RPC 2.0 Model Context Protocol server |

---

## 4. Model Context Protocol (MCP) Reference

| Tool | Parameters | Description |
|---|---|---|
| `chalkline_recon` | `target_dir?` | Query `miter-engine` for detected language conventions, error handlers, and async runtimes. |
| `chalkline_grill` | `prompt, spec_path?` | Execute non-interactive ambiguity analysis and extract decision holes. |
| `chalkline_compile_dag` | `spec_path, auto_link?` | Parse Markdown specification tasks and compile topological nodes into Storypole. |
| `chalkline_sync` | `spec_path` | Reconcile markdown task checklists with current Storypole ledger states. |

---

## 5. Canonical 9-Document Architecture

`chalkline` adheres strictly to the canonical **9-Document Architecture** (audited via `chalkline docs check`):
* **[`README.md`](./README.md):** Public elevator pitch, ecosystem architecture, installation, and CLI reference.
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Local toolchain prerequisites, tiered verification loop, and PR etiquette.
* **[`PRODUCT.md`](./PRODUCT.md):** Target personas, developer friction points, JTBD framework, and telemetry metrics.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Subsystem Mermaid topology, sub-crate boundaries, and AST dataflow pipelines.
* **[`SPEC.md`](./SPEC.md):** Behavioral invariants, state machines, formal boundary wire types, and exit codes.
* **[`DESIGN.md`](./DESIGN.md):** Architectural Decision Records (ADRs) explaining core design trade-offs.
* **[`ROADMAP.md`](./ROADMAP.md):** Active beta lifecycle status, completed capabilities, and future horizons.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Historical release entries and unreleased changes (Keep a Changelog format).
* **[`AGENTS.md`](./AGENTS.md):** Autonomous systems directives, non-negotiable engineering laws, and <=250 LOC ceilings.

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

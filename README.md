# `chalkline` — Interactive Specification Planner & Topological DAG Compiler

[![CI](https://github.com/gaugeblock/chalkline/actions/workflows/ci.yml/badge.svg)](https://github.com/gaugeblock/chalkline/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/chalkline.svg)](https://crates.io/crates/chalkline)
[![Documentation](https://docs.rs/chalkline/badge.svg)](https://docs.rs/chalkline)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

Part of the **[gaugeblock](https://gaugeblock.dev)** deterministic neurosymbolic suite.

In carpentry, a **chalk line** is snapped across rough wood to establish an exact, straight guide line *before* any cuts are made. In the `gaugeblock` ecosystem, `chalkline` snaps the architectural reference line before code generation begins:

1. **Reconnaissance (`recon`):** Queries `miter-engine` and the repository manifest to discover existing patterns (error crates, async runtime, conventions) so it never bothers the human with questions the code already answers.
2. **Ambiguity Griller (`grill`):** Identifies true `DecisionHole` architectural forks, presents tradeoff summaries with impacted symbols, and prompts the user via an interactive terminal UI (`inquire`) or stdio MCP.
3. **Cross-Language Contracts (`contracts_gen`):** Locks neutral `SharedTypeContract` schemas and exports JSON schemas for downstream `dado` polyglot code generation.
4. **Topological DAG Compiler (`compile-dag`):** Compiles approved spec tasks into sequenced, acyclic task nodes and dependencies directly within `storypole` (`.storypole/ledger.jsonl`).

---

## Workspace Layout

- **`chalkline`**: Root facade re-exporting `chalkline-core`.
- **`chalkline-core`**: Pure domain types, Markdown AST spec parser, recon engine, ambiguity griller, and storypole DAG compiler.
- **`chalkline-mcp`**: JSON-RPC 2.0 stdio Model Context Protocol (MCP) server.
- **`chalkline-cli`**: Clap binary entrypoint (`chalkline`).

---

## Installation & CLI Usage

```bash
cargo install chalkline-cli
```

### 1. Interactive `/grill-me` Session
```bash
# Interactive terminal prompt resolving decision holes
chalkline grill "Add persistent API rate limiting"

# Auto-compile into storypole upon completion
chalkline grill --compile
```

### 2. Codebase Reconnaissance
```bash
chalkline recon
chalkline recon --json
```

### 3. Compile Spec to Storypole DAG
```bash
chalkline compile-dag --spec SPEC.md
```

### 4. Sync Task Checklists
```bash
chalkline sync --spec SPEC.md
```

### 5. Stdio Model Context Protocol (MCP) Server
```bash
chalkline mcp
```

---

## License

Licensed under Apache License 2.0.

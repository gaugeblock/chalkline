# AGENTS.md — Operational Directives for `chalkline`

Operational instructions and runtime guardrails for autonomous systems engineering agents working in this repository.

---

## 1. Project Mission

`chalkline` is the architectural specification planner, ambiguity griller, and 9-document repository architecture engine for the `gaugeblock` suite. It snaps the reference line before code generation:
1. Recons codebase conventions via `miter-engine` without context burning.
2. Identifies and resolves irreducible `DecisionHole` forks before code generation.
3. Compiles verified markdown task checklists into acyclic DAGs in `storypole`.
4. Audits and synchronizes the canonical 9-document repository architecture against code AST.

There is no model on the query path for `recon` or `docs check`. Every answer is a deterministic function of the repository tree.

---

## 2. Architecture & Crate Boundaries

Cargo workspace. Do not blur crate responsibilities:

* `crates/chalkline-core`: Pure domain types, spec parsing (`pulldown-cmark`), recon engine, decision griller, and documentation drift engine. No stdio CLI interaction, no terminal prompts.
* `crates/chalkline-cli`: Clap 4.5 CLI binary entrypoint (`chalkline`), human terminal formatting, interactive `inquire` prompts. Transports only; delegates all analysis to `chalkline-core`.
* `crates/chalkline-mcp`: Stdio JSON-RPC 2.0 Model Context Protocol server exposing `chalkline_recon`, `chalkline_grill`, `chalkline_compile_dag`, and `chalkline_sync`.
* Root package (`src/lib.rs`): Ergonomic facade re-exporting `chalkline-core`.

---

## 3. Invariant Rules (DO NOT BREAK)

### 1. Pure Markdown Invariance
* Do NOT use HTML comments (`<!-- CHALKLINE:... -->`) to delimit sections.
* All dynamic section synchronization must target native Markdown Heading AST nodes via `pulldown-cmark` (`replace_heading_section`).
* Hand-crafted narrative and diagrams outside the targeted heading must remain 100% untouched.

### 2. Acyclic DAG Invariants
* `chalkline compile-dag` must validate that the parsed task graph is acyclic before writing events to `storypole`.
* Cyclic dependencies must yield a typed `CompileError::CyclicDependency`.

### 3. Safe Error Handling & Observability
* Library code in `chalkline-core` must use `thiserror` (`ChalklineError`).
* `.unwrap()` and `.expect()` are strictly forbidden in production library paths (enforced via `clippy::unwrap_used = "deny"`).
* Binary/CLI entrypoints may use `anyhow::Result`.
* Use `tracing` for structured logging. Never print to `stdout` in `--json` or `--mcp` modes.

---

## 4. Non-Negotiable Operational Laws

1. **Hard 250-Line File Limit:**
   * No `.rs` source file may exceed **250 lines of code** (including comments and tests).
   * If a file approaches 200 lines, split it into submodules (`mod.rs` or `<module>/<submodule>.rs`).
2. **Unix Philosophy & Standalone Value:**
   * `chalkline` must compile, test, and provide a fully functional CLI and MCP experience standalone without requiring the rest of the suite to be running.
3. **Zero Inline Scope Creep (storypole defer):**
   * Unrelated defects, missing AST extractors, or debt discovered during a task must never be fixed inline. Record them via `storypole defer --kind <bug|tech-debt> --title "..."`.
4. **Tiered Verification Matrix:**
   * Tier 1: `cargo check --manifest-path crates/chalkline-core/Cargo.toml`
   * Tier 2: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --workspace`
   * Tier 3: `chalkline docs check -d .`
5. **Atomic Conventional Commits:**
   * Commits must follow Conventional Commits v1.0.0: `<type>(<scope>): <summary>` (e.g. `feat(chalkline-core): implement drift rule`).
6. **Branch Isolation & Plan Persistence:**
   * All non-trivial tickets must be developed on dedicated feature branches (`feat/...`).
   * Design blueprints must be saved to `.storypole/plans/<KEY>.md`.

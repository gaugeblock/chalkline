# GB-102: Chalkline Documentation Engine Evolution & Suite Verification Harness

## Architectural Directives & Locked Decisions
1. **Native Markdown AST Targeting:** Do NOT use HTML comments. Use `pulldown-cmark` heading range parsing to target standard Markdown sections (`## Crate Topology`, `## CLI Reference`, `## Boundary Contracts`, `## [Unreleased]`).
2. **Boundary & Wire Type Scoping:** Scope `enforce_public_type_contracts` to boundary types (Serialize/Deserialize, thiserror enums, Clap commands, lib facade exports).
3. **Dogfooding with Chalkline's 9 Documents:** Elevate all 9 documents in `chalkline` to gold standard (Mermaid diagrams, CLI/MCP reference, ADRs, AGENTS.md directives) as the first certified pass of `chalkline docs check`.

## Phased Execution Steps
1. `gb-103-workspace-audit` | Add `--workspace` / `--all` recursive discovery to `chalkline docs check` with consolidated matrix output | `cargo test -p chalkline-cli`
2. `gb-104-ast-drift-policies` | Implement boundary type AST matching, Clap CLI flag inspection, AGENTS laws verification, and link integrity in `chalkline-core` | `cargo test -p chalkline-core`
3. `gb-105-docs-sync-templates` | Implement `chalkline docs sync` via `pulldown-cmark` heading AST rewriting and upgrade `templates.rs` | `cargo test -p chalkline-core`
4. `gb-119-chalkline-docs-elevation` | Elevate all 9 canonical docs in `chalkline` to gold standard matching `miter` | `chalkline docs check -d chalkline`
5. `gb-99-tier3-gate` | Codify `chalkline docs check --workspace` in root `AGENTS.md` and release pre-flight | `chalkline docs check --workspace`

# `chalkline` — Architectural Design & Decisions

> **Architectural Decision Records (ADRs), Trade-Off Analyses & Design Invariants**

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic suite.

---

## 1. Architectural Decision Records (ADRs)

### ADR-001: Canonical 9-Document Architecture & Orthogonal Separation of Concerns
* **Status:** Accepted
* **Context:** Software documentation in multi-agent and human engineering workflows routinely drifts, mixes operational directives with product roadmaps, or bloats monolithic README files into multi-thousand-line mazes.
* **Decision:** Enforce a strict, mutually exclusive 9-document architecture (`README`, `CONTRIBUTING`, `PRODUCT`, `ARCHITECTURE`, `SPEC`, `DESIGN`, `ROADMAP`, `CHANGELOG`, `AGENTS`) across every member repository in `gaugeblock`. Every document has a discrete job-to-be-done and clear ownership boundary.
* **Consequences:**
  * Clean context slicing for AI coding agents (agents read only `AGENTS.md` and `SPEC.md` for tasks instead of 5,000 lines of mixed documentation).
  * Audited automatically via `chalkline docs check`.
* **Alternatives Rejected:**
  * Single monolithic `README.md`: Overloads context windows and conflates contributor guides with product pitches.
  * Ad-hoc docs folders without schemas: Degrades into unmaintained orphan markdown files.

---

### ADR-002: Native Markdown AST Heading Range Targeting vs HTML Comment Markers
* **Status:** Accepted (Locked via `GB-102` / `GB-105`)
* **Context:** Code-grounded documentation sections (e.g. `## Crate & Package Topology`, `## CLI Reference`, `## Formal Type Contracts`) need automated synchronization as code AST changes. Many existing tools use non-standard HTML comment markers (e.g. `<!-- CHALKLINE:CRATES:START -->`).
* **Decision:** Do NOT use HTML comments. Target native Markdown AST heading ranges using `pulldown-cmark`. When synchronizing, `chalkline docs sync` locates the target `Heading` node (e.g. `## Crate & Package Topology`) and surgical-replaces the byte range up to the next heading of equal or higher level.
* **Consequences:**
  * All documentation remains 100% clean, standard, idiomatic Markdown readable on GitHub, Crates.io, and static site generators without syntax pollution.
  * Exact byte offsets from `into_offset_iter()` prevent accidental mutation of surrounding prose.
* **Alternatives Rejected:**
  * HTML comment tags (`<!-- START -->` / `<!-- END -->`): Pollutes markdown source and easily gets corrupted by human edits.
  * Whole-file clobbering: Destroys hand-crafted developer narrative, diagrams, and domain context.

---

### ADR-003: Boundary & Wire Type Scoping for Specification Drift Verification
* **Status:** Accepted (Locked via `GB-104`)
* **Context:** Auditing that all public structs and functions in Rust/TS are documented in `SPEC.md` can cause low-value, noisy dumps of trivial internal helper structs.
* **Decision:** Scope `enforce_public_type_contracts` to boundary and contract types:
  1. Types deriving `serde::Serialize` or `serde::Deserialize` (wire schemas).
  2. Strongly typed error enums (`thiserror`).
  3. CLI entry structs (`clap::Parser` and `clap::Subcommand`).
  4. Public types exported from the root `lib.rs` facade.
* **Consequences:**
  * `SPEC.md` remains high-density, focused on formal system boundaries, wire protocols, and error conditions.
  * Prevents agent token waste on trivial internal structs.
* **Alternatives Rejected:**
  * All `pub` items: Forces massive, unreadable tables of hundreds of trivial types.
  * Zero automated type checks: Allows API wire formats to drift silently from documentation.

---

### ADR-004: Decoupled Reconnaissance via `miter-engine`
* **Status:** Accepted
* **Context:** `chalkline` needs to know codebase conventions (testing crates, logging, async runtimes) before grilling prompts or compiling specs.
* **Decision:** Delegate AST indexing and dependency inspection strictly to `miter-engine`. `chalkline` owns zero tree-sitter grammars or redb databases; it consumes the normalized `WorkspaceInventory` contract.
* **Consequences:**
  * Zero code duplication between `miter` and `chalkline`.
  * Fast sub-50ms convention detection.
* **Alternatives Rejected:**
  * Re-implementing regex or AST parsing inside `chalkline`: Violates Unix philosophy and creates divergent parsing logic.

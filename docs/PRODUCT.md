# `chalkline` — Product Vision & Strategy

> **Precision Specification Planner, Ambiguity Griller & 9-Document Architecture Engine**

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic systems engineering suite.

---

## 1. Executive Summary

When AI coding agents (Claude Code, Cursor, Windsurf, Cline) are assigned complex, multi-file software engineering tasks, they frequently fail at the design and planning phase:
1. **Unbounded Ambiguity:** Agents make arbitrary, silent assumptions about critical architectural decisions (e.g., choice of database, error crate, wire format) without consulting humans.
2. **Context Window Combustion:** Agents burn 80%+ of their context window asking questions about conventions that are already answered in the existing codebase.
3. **Execution Paralysis:** Agents generate linear todo lists with circular dependencies, missing edge links, or unverified intermediate steps.
4. **Documentation Drift:** Architecture documents, specs, and READMEs decay silently as code AST evolves, turning repository docs into misleading fiction.

`chalkline` is the deterministic architectural reference guide snapped before code is written. It combines AST reconnaissance (`miter-engine`), interactive decision resolution (`inquire` TUI / stdio MCP), topological DAG compilation (`storypole`), and AST-grounded 9-document verification to ensure software is specified deterministically before a single line of production code is modified.

---

## 2. Target Personas & Customer Journeys

### Persona A: The Systems Architect (Human Technical Lead)
* **Profile:** Staff/Principal Engineer directing autonomous agent swarms or engineering teams across multi-crate Rust/TypeScript codebases.
* **Friction Points:**
  * Exhausted by reviewing PRs that choose the wrong database driver, introduce incompatible serialization crates, or violate layered architectural boundaries.
  * Frustrated by stale `README.md` flags and `ARCHITECTURE.md` crate diagrams that drifted 6 months ago.
* **The `chalkline` Solution:**
  * Interactively grilled on only the irreducible `DecisionHole` forks via `chalkline grill`.
  * Reviews machine-compiled, acyclic task DAGs in `storypole` before execution begins.
  * Runs `chalkline docs check --workspace` in CI to fail PRs with semantic documentation drift.

### Persona B: The Autonomous Coding Agent (Antigravity, Claude Code, Windsurf)
* **Profile:** LLM-driven autonomous systems engineering agent executing multi-step tasks across the meta-harness.
* **Friction Points:**
  * Wastes dozens of round-trips guessing project conventions.
  * Makes unapproved architectural changes that get rejected during review.
  * Cannot reliably synchronize markdown checkbox state with underlying backlog stores.
* **The `chalkline` Solution:**
  * Invokes `chalkline_recon` to instantly learn existing framework, async, and error conventions.
  * Compiles approved specification markdown into deterministic `storypole` task DAGs via `chalkline_compile_dag`.
  * Dogfoods `chalkline_sync` to keep specifications and ledgers in lockstep.

---

## 3. Jobs to Be Done (JTBD) Framework

| When I am... | I want to... | So that I can... |
|---|---|---|
| Handing a complex feature prompt to an AI agent | Automatically recon the repo and grill the prompt for unresolved design forks | Ensure the agent builds the right architecture without hallucinating external dependencies. |
| Reviewing a multi-step design specification (`SPEC.md`) | Compile the task checklist into an acyclic task DAG | Enforce topological dependencies and prevent race conditions during execution. |
| Committing or releasing code across multi-repository workspaces | Audit all 9 canonical documents against code AST and Clap CLI definitions | Guarantee 100% documentation fidelity with zero drift across all repositories. |
| Refactoring a legacy monolith into modular submodules | Automatically update crate listings, CLI command tables, and public type contracts | Avoid manual documentation maintenance toil and preserve human narrative. |

---

## 4. Competitive Differentiation & Positioning

| Dimension | Generic LLM Chat / Agents | Traditional Linters & Scaffolding | `chalkline` Neurosymbolic Architecture |
|---|---|---|---|
| **Ambiguity Detection** | Hallucinates an arbitrary choice silently | Cannot inspect semantic prose | Identifies formal `DecisionHole` forks with impacted symbols. |
| **Codebase Recon** | Greps entire repository burning context | None | AST-grounded query via `miter-engine` (<50ms). |
| **Task Planning** | Unstructured markdown checkboxes | Issue trackers without DAG guarantees | Topological DAG compiled directly into Git-native `storypole`. |
| **Documentation Audit** | Manual or unverified | Lints markdown syntax only | Audits prose against code AST, Clap CLI flags, and Git commits. |
| **Section Regeneration** | Clobbers files or requires HTML comment tags | None | Native Markdown AST heading targeting via `pulldown-cmark`. |

---

## 5. Success Metrics & Telemetry

* **Zero Silent Architectural Assumptions:** 100% of discovered `DecisionHole` forks are either auto-resolved via AST recon or explicitly resolved by the user.
* **Sub-Second Suite Verification:** `chalkline docs check --workspace` audits all 11 workspace targets in under 1.5 seconds.
* **Zero Documentation Drift:** 0% drift between code AST public types, Clap CLI subcommands, and canonical documents in verified repositories.

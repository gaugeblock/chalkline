# `chalkline` — Roadmap & Horizons

> **Milestones, Completed Capabilities & Future Horizons**

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic suite.  
Current Lifecycle: `0.1.0-beta.3` | Git tag format: `v<version>`

---

## 1. Completed Capabilities (v0.1.0-alpha.1 -> v0.1.0-beta.3)

- [x] **AST Codebase Reconnaissance:** `miter-engine` integration to extract async runtimes, logging, and error conventions (`chalkline recon`).
- [x] **Interactive Ambiguity Griller:** DecisionHole extraction and interactive `inquire` TUI resolution (`chalkline grill`).
- [x] **Topological Task DAG Compiler:** Parsing `SPEC.md` tasks and generating acyclic nodes in `storypole` (`chalkline compile-dag`).
- [x] **Canonical 9-Document Architecture:** Static auditing against 9-document topology and `.gaugeblock/docs.toml` (`chalkline docs check`).
- [x] **Brownfield Document Disentangler:** Migrating bloated monolithic README files into 9 canonical documents (`chalkline docs migrate`).
- [x] **Multi-Repo Workspace Auditing:** Consolidated suite-wide 9-document compliance matrix via `--workspace` / `--all` (`GB-103`).
- [x] **Semantic AST Drift Verification:** Scoped boundary type auditing, Clap CLI flag inspection, AGENTS laws verification, and link integrity (`GB-104`).
- [x] **Native Markdown AST Sync:** Pure Markdown AST heading replacement via `pulldown-cmark` and production scaffolding templates (`GB-105`).

---

## 2. Active Milestone (v0.1.0-beta.4 — Documentation Suite Tier 3 Certification)

- [ ] **Dogfood Chalkline Suite Certification:** Certify all 9 documents across `chalkline` with zero missing files and zero AST drift (`GB-119`).
- [ ] **Tier 3 Verification Gate Integration:** Codify `chalkline docs check --workspace` in the meta-harness `AGENTS.md` and `outfeed release` pre-flight gate (`GB-99`).
- [ ] **Benchwork CAD Console Integration:** Expose `chalkline` docs audit reports and decision grill sessions directly in `benchwork` Panel 1 and Panel 4.

---

## 3. Future Horizons (v0.2.0+)

- [ ] **Interactive Visual Griller:** Web-based interactive DecisionHole resolution in `benchwork` with real-time dependency graph preview.
- [ ] **Cross-Language Wire Contract Verification:** Bidirectional schema drift verification comparing Rust structs with TypeScript interfaces and SQL migrations.
- [ ] **Automatic Spec Diff Generator:** Generating architectural diffs between Git branches to summarize design changes before PR reviews.

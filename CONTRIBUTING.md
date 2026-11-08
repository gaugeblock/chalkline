# Contributing to `chalkline`

Thank you for contributing to `chalkline`! `chalkline` is an open-source, high-reliability specification planner and documentation engine in the **[`gaugeblock`](https://gaugeblock.dev)** ecosystem.

---

## 1. Prerequisites & Toolchain

To ensure bit-for-bit reproducibility across all machines, ensure your local workstation has:

* **Rust:** `1.90.0` or later (Rust 2024 Edition)
* **Cargo Nextest:** `cargo-nextest` preferred runner (`cargo install cargo-nextest --locked`)
* **Clippy & Rustfmt:** Components installed via `rustup component add clippy rustfmt`
* **Node.js (Optional):** `>=20.0.0` if interacting with sibling web consoles (`benchwork`)

---

## 2. Inner Development Loop (Tiered Verification)

Development adheres to the strict **Tiered Verification Matrix**:

### Tier 1: Inner Loop (Rapid Iteration — <2s)
Run targeted checks on the active crate while writing code:
```bash
cargo check --manifest-path crates/chalkline-core/Cargo.toml
# or run a targeted test:
cargo test --manifest-path crates/chalkline-core/Cargo.toml --lib docs::tests
```

### Tier 2: Atomic Commit Gate
Before creating any commit, verify that formatting, lints, and unit tests pass:
```bash
cargo fmt --check --manifest-path Cargo.toml
cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path Cargo.toml --workspace
```

### Tier 3: Task Gate
Before marking any task `Verified` in `storypole`:
```bash
chalkline docs check -d .
cargo test --manifest-path Cargo.toml --workspace
```

---

## 3. Non-Negotiable Coding Standards

1. **Hard 250-Line File Limit:**
   * No `.rs` source file may exceed **250 lines of code** (including comments and tests).
   * If a file approaches 200 lines, split it into submodules (`mod.rs` or `<module>/<submodule>.rs`).
2. **Strict Error Handling:**
   * Production code in `chalkline-core` MUST use strongly typed `thiserror` enums (`ChalklineError`).
   * `.unwrap()` and `.expect()` are strictly forbidden in production library paths (enforced via `clippy::unwrap_used = "deny"`).
3. **Conventional Commits:**
   * Commits MUST follow Conventional Commits v1.0.0: `<type>(<scope>): <summary>`
   * Scopes: `chalkline-core`, `chalkline-cli`, `chalkline-mcp`, `docs`, `workspace`.
   * Include Storypole task footers when applicable (`Task: GB-XXX`).
4. **Zero Inline Scope Creep:**
   * Never fix unrelated bugs inline. Record out-of-scope defects via `storypole defer`.

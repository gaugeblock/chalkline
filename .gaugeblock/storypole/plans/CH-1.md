# GB-91: Fix missing reasoning_effort in chalkline-core griller InferenceRequest initializer

## Context & Root Cause
In `sawhorse-core` (GB-81 / GB-84), `InferenceRequest` gained a new field:
`pub reasoning_effort: Option<ReasoningEffort>`.
In `chalkline/crates/chalkline-core/src/griller.rs:98-113`, `build_planner_request` initialized `InferenceRequest` with a direct struct literal without `reasoning_effort`, causing rustc error `E0063`.

## Implementation Steps
1. In `chalkline/crates/chalkline-core/src/griller.rs`:
   - Add `reasoning_effort: None` to the `InferenceRequest` struct initialization in `build_planner_request`.
2. Verify:
   - Run `cargo check -p chalkline-core` and `cargo test -p chalkline-core`.
   - Run Tier 2 checks on `chalkline`: `cargo fmt --check -p chalkline-core`, `cargo clippy -p chalkline-core`, `cargo test -p chalkline-core`.
3. Commit atomically with Conventional Commits: `fix(chalkline-core): initialize reasoning_effort in build_planner_request`.


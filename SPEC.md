# Behavioral Specifications & Invariants

## Formal Type Contracts

- **`error::ChalklineError`** (`enum`): Master error enumeration for `chalkline-core`.
- **`args::Cli`** (`struct`): Public interface entity
- **`args::Commands`** (`enum`): Available subcommands for `chalkline`.
- **`dag_compiler::CompilationReceipt`** (`struct`): Summary of tasks and edges compiled into storypole.
- **`args::CompileArgs`** (`struct`): Arguments for `chalkline compile-dag`.
- **`model::ContractField`** (`struct`): A single field inside a `SharedTypeContract`.
- **`contracts_gen::ContractsGenerator`** (`struct`): Cross-language contract generator and exporter.
- **`dag_compiler::DagCompiler`** (`struct`): DAG compiler for specification tasks.
- **`model::DecisionHole`** (`struct`): An architectural decision hole identifying an ambiguity to resolve.
- **`model::DecisionOption`** (`struct`): A specific choice for resolving a decision hole.
- **`drift::DocAuditItem`** (`struct`): Audit result for a single document.
- **`drift::DocAuditReport`** (`struct`): Aggregated documentation audit report.
- **`config::DocEntryConfig`** (`struct`): Individual document configuration entry.
- **`migration::DocMigrationPlan`** (`struct`): Proposed plan for disentangling a monolithic document into the 9 canonical documents.
- **`drift::DocStatus`** (`enum`): Status of a documentation file.
- **`config::DocType`** (`enum`): Canonical documentation types managed by gaugeblock.
- **`args_docs::DocsArgs`** (`struct`): Arguments for `chalkline docs`.
- **`args_docs::DocsCheckArgs`** (`struct`): Arguments for `chalkline docs check`.
- **`args_docs::DocsCommands`** (`enum`): Subcommands for `chalkline docs`.
- **`config::DocsConfig`** (`struct`): Master configuration mapped from `.gaugeblock/docs.toml`.

//! # `chalkline-core`
//!
//! Interactive specification planner, ambiguity griller (`/grill-me`),
//! codebase reconnaissance engine, cross-language contract locker,
//! and topological task DAG compiler for `storypole`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod contracts_gen;
pub mod dag_compiler;
pub mod docs;
pub mod error;
pub mod griller;
pub mod model;
pub mod recon;
pub mod spec_parser;

pub use contracts_gen::ContractsGenerator;
pub use dag_compiler::{CompilationReceipt, DagCompiler};
pub use docs::{
    DocAuditItem, DocAuditReport, DocEntryConfig, DocMigrationPlan, DocStatus, DocType, DocsConfig,
    DocsPoliciesConfig, DocumentEntries, MarkdownSection, render_template, verify_docs_drift,
};
pub use error::{ChalklineError, Result};
pub use griller::Griller;
pub use model::{
    ContractField, DecisionHole, DecisionOption, SharedTypeContract, SpecDoc, SpecTask,
};
pub use recon::{ReconEngine, ReconReport};
pub use spec_parser::SpecParser;

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

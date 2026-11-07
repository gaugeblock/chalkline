//! DAG compiler: a specification's tasks and edges, emitted into a sink the caller supplies.
//!
//! The compiler used to open a storypole store and write rows itself, which welded chalkline's domain
//! core to another product's engine (`GB-204`). It now emits into a port: the domain core decides *what*
//! tasks and edges a specification implies, and the integrator (`-cli`, `-mcp`) decides *where* they are
//! written. Only an integrator may depend on storypole, and this trait is the seam that lets it.

#![forbid(unsafe_code)]

use crate::error::Result;
use crate::model::SpecDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A task a specification implies, before any engine has seen it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledTask {
    /// The spec's own key (`T1`, `GB-12`), so edges within one document can refer to each other.
    pub key: String,
    /// Task title, taken from the spec verbatim.
    pub title: String,
    /// Task description, or a generated one naming the specification.
    pub description: String,
}

/// Where compiled tasks go.
///
/// Deliberately two methods: create a task, and declare that one task is blocked by another. A wider
/// port would let the compiler reach for behaviour a specification cannot express, and every method
/// here is one the sink author has to keep working forever.
pub trait TaskSink {
    /// The sink's own handle for a created task, used only to link edges.
    type Id: Copy + Eq + core::hash::Hash;

    /// Creates `task`, returning its id and its ticket key, in that order.
    fn create(&mut self, task: &CompiledTask) -> Result<(Self::Id, String)>;

    /// Records that `child` is blocked by `blocker`.
    fn link_blocked_by(&mut self, child: Self::Id, blocker: Self::Id) -> Result<()>;
}

/// Summary of tasks and edges compiled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilationReceipt {
    /// Total tasks compiled.
    pub tasks_compiled: usize,
    /// Total dependency edges linked.
    pub edges_linked: usize,
    /// Ticket keys the sink assigned, e.g. `["GB-3", "GB-4"]`.
    pub ticket_keys: Vec<String>,
}

/// DAG compiler for specification tasks.
#[derive(Debug, Default)]
pub struct DagCompiler;

impl DagCompiler {
    /// Creates a new `DagCompiler`.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Compiles every task in `doc` into `sink`, creating all tasks before linking any edge.
    ///
    /// Two passes because an edge may name a task the specification lists later: one pass to collect
    /// every id, then one to link. An edge naming a key the spec never declared is skipped rather than
    /// failed, which is the behaviour this compiler has always had and what a partially written spec
    /// relies on.
    pub fn compile(&self, sink: &mut impl TaskSink, doc: &SpecDoc) -> Result<CompilationReceipt> {
        let mut ids = HashMap::new();
        let mut ticket_keys = Vec::new();

        for spec_task in &doc.tasks {
            let task = CompiledTask {
                key: spec_task.key.clone(),
                title: spec_task.title.clone(),
                description: spec_task
                    .description
                    .clone()
                    .unwrap_or_else(|| format!("Task compiled from spec: {}", doc.title)),
            };
            let (id, ticket_key) = sink.create(&task)?;
            ids.insert(spec_task.key.clone(), id);
            ticket_keys.push(ticket_key);
        }

        let mut edges_linked = 0;
        for spec_task in &doc.tasks {
            if let Some(&child) = ids.get(&spec_task.key) {
                for blocker_key in &spec_task.blocked_by {
                    if let Some(&blocker) = ids.get(blocker_key) {
                        sink.link_blocked_by(child, blocker)?;
                        edges_linked += 1;
                    }
                }
            }
        }

        Ok(CompilationReceipt {
            tasks_compiled: doc.tasks.len(),
            edges_linked,
            ticket_keys,
        })
    }
}

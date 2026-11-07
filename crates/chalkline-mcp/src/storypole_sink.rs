//! Storypole adapter for chalkline's `TaskSink` port (`GB-204`, `GB-284`).
//!
//! Lives in the MCP integrator rather than in `chalkline-core` because the edge to storypole is the whole
//! thing being removed: only an integrator may hold it. `chalkline-cli` already depends on this crate, so
//! one adapter serves both front-ends and neither duplicates it.

use chalkline_core::dag_compiler::{CompiledTask, TaskSink};
use chalkline_core::{ChalklineError, Result};
use std::path::Path;
use storypole_core::model::task::{EdgeKind, TaskKind, TaskScope};
use storypole_core::ops::graph::link_tasks;
use storypole_core::ops::mutations::{CreateTaskParams, add_task};
use storypole_core::store::StorypoleStore;

/// Writes compiled tasks into a storypole ledger at a repository root.
#[derive(Debug)]
pub struct StorypoleSink {
    store: StorypoleStore,
}

impl StorypoleSink {
    /// Opens the ledger at `repo_root`.
    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self> {
        let store = StorypoleStore::open(repo_root.as_ref())
            .map_err(|error| ChalklineError::Storypole(error.to_string()))?;
        Ok(Self { store })
    }
}

impl TaskSink for StorypoleSink {
    type Id = uuid::Uuid;

    fn create(&mut self, task: &CompiledTask) -> Result<(Self::Id, String)> {
        let params = CreateTaskParams {
            title: task.title.clone(),
            description: task.description.clone(),
            kind: TaskKind::Feature,
            priority: 1,
            scope: TaskScope::default(),
            acceptance_criteria: Vec::new(),
            status: None,
            edges: Vec::new(),
        };
        let node = add_task(&mut self.store, params)
            .map_err(|error| ChalklineError::Storypole(error.to_string()))?;
        Ok((node.id, node.ticket_key))
    }

    fn link_blocked_by(&mut self, child: Self::Id, blocker: Self::Id) -> Result<()> {
        link_tasks(&mut self.store, child, blocker, EdgeKind::BlockedBy)
            .map_err(|error| ChalklineError::Storypole(error.to_string()))
    }
}

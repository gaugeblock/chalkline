//! DAG compiler that maps specification tasks directly into storypole tasks and edges.

#![forbid(unsafe_code)]

use crate::error::{ChalklineError, Result};
use crate::model::SpecDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use storypole_core::model::task::{EdgeKind, TaskKind, TaskScope};
use storypole_core::ops::graph::link_tasks;
use storypole_core::ops::mutations::{CreateTaskParams, add_task};
use storypole_core::store::StorypoleStore;

/// Summary of tasks and edges compiled into storypole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilationReceipt {
    /// Total tasks compiled.
    pub tasks_compiled: usize,
    /// Total dependency edges created.
    pub edges_linked: usize,
    /// List of created storypole ticket keys (e.g., ["GB-3", "GB-4"]).
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

    /// Compiles all tasks in `doc` into the storypole store at `repo_root`.
    pub fn compile(
        &self,
        repo_root: impl AsRef<Path>,
        doc: &SpecDoc,
    ) -> Result<CompilationReceipt> {
        let root = repo_root.as_ref();
        let mut store =
            StorypoleStore::open(root).map_err(|e| ChalklineError::Storypole(e.to_string()))?;

        let mut key_to_uuid = HashMap::new();
        let mut ticket_keys = Vec::new();

        // 1. Create all task nodes first
        for spec_task in &doc.tasks {
            let params = CreateTaskParams {
                title: spec_task.title.clone(),
                description: spec_task
                    .description
                    .clone()
                    .unwrap_or_else(|| format!("Task compiled from spec: {}", doc.title)),
                kind: TaskKind::Feature,
                priority: 1,
                scope: TaskScope::default(),
                acceptance_criteria: Vec::new(),
                status: None,
                edges: Vec::new(),
            };

            let node = add_task(&mut store, params)
                .map_err(|e| ChalklineError::Storypole(e.to_string()))?;

            key_to_uuid.insert(spec_task.key.clone(), node.id);
            ticket_keys.push(node.ticket_key);
        }

        // 2. Link blocked_by edges
        let mut edges_linked = 0;
        for spec_task in &doc.tasks {
            if let Some(&child_id) = key_to_uuid.get(&spec_task.key) {
                for blocker_key in &spec_task.blocked_by {
                    if let Some(&blocker_id) = key_to_uuid.get(blocker_key) {
                        link_tasks(&mut store, child_id, blocker_id, EdgeKind::BlockedBy)
                            .map_err(|e| ChalklineError::Storypole(e.to_string()))?;
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

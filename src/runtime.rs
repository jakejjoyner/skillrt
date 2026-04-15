//! Skill execution runtime.
//!
//! v0.1 is a stub. The real runtime will:
//!   - resolve `inputs` against user-provided JSON
//!   - enforce `permissions` (network/fs/env)
//!   - invoke declared `dependencies.tools` in a child process
//!   - capture stdout/stderr/exit, validate against `outputs.schema`
//!   - emit structured telemetry (OTLP-compatible)

use crate::error::Result;
use crate::skill::Skill;

pub struct RunOptions {
    pub inputs: serde_json::Value,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            inputs: serde_json::json!({}),
        }
    }
}

pub struct RunResult {
    pub output: serde_json::Value,
    pub stdout: String,
    pub stderr: String,
}

/// Execute a skill. Prose skills are not executable; callers should feed the
/// prose body to their agent as reference material instead.
pub fn run(_skill: &Skill, _opts: RunOptions) -> Result<RunResult> {
    // TODO (v0.2): implement. For now, the CLI prints the skill body so agents
    // can at least read it via `skill run`.
    Ok(RunResult {
        output: serde_json::json!(null),
        stdout: String::new(),
        stderr: "runtime execution not yet implemented in v0.1".into(),
    })
}

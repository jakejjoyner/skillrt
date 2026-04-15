//! YAML frontmatter schema for structured skills (v0.1 DRAFT).
//!
//! The schema is intentionally minimal in v0.1. Fields marked `optional` may
//! become required in v0.2 once we see how skills are authored in practice.

use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Top-level frontmatter for a structured SKILL.md.
///
/// Prose-mode soapstones have no frontmatter at all; see `parser::parse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    /// Skill identifier. Must be lowercase, kebab-case, globally unique per namespace.
    pub name: String,

    /// SemVer.
    pub version: Version,

    /// One-line description shown in registry listings (<=120 chars recommended).
    pub description: String,

    #[serde(default)]
    pub authors: Vec<String>,

    /// SPDX identifier (e.g., "MIT", "Apache-2.0").
    #[serde(default = "default_license")]
    pub license: String,

    /// Declares this document conforms to the skillrt skill format.
    #[serde(default)]
    pub runtime: RuntimeRequirement,

    #[serde(default)]
    pub inputs: Vec<InputSpec>,

    #[serde(default)]
    pub outputs: Option<OutputSpec>,

    #[serde(default)]
    pub dependencies: Dependencies,

    #[serde(default)]
    pub permissions: Permissions,

    #[serde(default)]
    pub tags: Vec<String>,

    /// Reserved for future extensions. Unknown fields are preserved.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeRequirement {
    #[serde(rename = "type", default = "default_runtime_type")]
    pub kind: String,

    /// Minimum skillrt version this skill requires.
    #[serde(default)]
    pub min_version: Option<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: InputKind,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub default: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputKind {
    String,
    Number,
    Boolean,
    File,
    Url,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSpec {
    #[serde(rename = "type")]
    pub kind: OutputKind,
    /// Optional JSON Schema describing the output shape.
    #[serde(default)]
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputKind {
    Text,
    Json,
    File,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dependencies {
    /// MCP servers this skill expects to be available.
    #[serde(default, rename = "mcp-servers")]
    pub mcp_servers: Vec<String>,

    /// Other skills this skill depends on, `name@version` (semver range).
    #[serde(default)]
    pub skills: Vec<String>,

    /// External CLI tools this skill shells out to (e.g. `bash`, `python`, `curl`).
    #[serde(default)]
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Permissions {
    /// Allowed network destinations. Use `"*"` for any; empty = none.
    #[serde(default)]
    pub network: Vec<String>,

    /// Filesystem paths this skill may read/write.
    #[serde(default)]
    pub filesystem: Vec<String>,

    /// Environment variables this skill may read.
    #[serde(default)]
    pub env: Vec<String>,
}

fn default_license() -> String {
    "UNLICENSED".to_string()
}

fn default_runtime_type() -> String {
    "markdown-skill".to_string()
}

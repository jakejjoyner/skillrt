//! skillrt - runtime and spec for executable markdown skills consumed by AI agents.
//!
//! Supports two content modes:
//!   - Prose (Soapstones-compatible): free-form markdown, no frontmatter required
//!   - Structured: typed SKILL.md with frontmatter schema, inputs, outputs, dependencies
//!
//! Public API stabilizes in v0.2. Until then everything is subject to change.

pub mod error;
pub mod frontmatter;
pub mod parser;
pub mod registry;
pub mod runtime;
pub mod skill;

pub use error::{Result, SkillError};
pub use skill::{Skill, SkillBody, SkillKind};

//! The top-level Skill type.

use crate::frontmatter::Frontmatter;

/// What kind of content this skill carries.
///
/// The runtime can consume both; prose skills are shown to the agent as
/// reference material (Soapstones-style), structured skills are invoked
/// with inputs and return a typed output.
#[derive(Debug, Clone)]
pub enum SkillKind {
    /// Free-form markdown, no frontmatter. Interpreted by the agent.
    /// Compatible with Cal Paterson's Soapstones format.
    Prose,

    /// Typed, executable skill with frontmatter metadata.
    Structured,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub kind: SkillKind,
    pub frontmatter: Option<Frontmatter>,
    pub body: SkillBody,
    pub source_path: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone)]
pub struct SkillBody {
    /// The raw markdown body (everything after frontmatter).
    pub markdown: String,
}

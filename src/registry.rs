//! Local on-disk skill storage.
//!
//! v0.1 is local-only: `~/.skillrt/skills/<name>/<version>/SKILL.md`.
//! v0.2 will add a hosted registry (skills.dev or similar) with HTTP fetch.

use crate::error::{Result, SkillError};
use crate::parser;
use crate::skill::Skill;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

/// Returns the on-disk root where installed skills live.
pub fn root() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "skillrt", "skillrt")
        .ok_or_else(|| SkillError::Other("cannot resolve user data dir".into()))?;
    let root = dirs.data_dir().join("skills");
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

/// Load all skills installed locally.
pub fn list() -> Result<Vec<Skill>> {
    let root = root()?;
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(&root).max_depth(3) {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry.file_name() == "SKILL.md"
            && let Ok(skill) = load(entry.path())
        {
            out.push(skill);
        }
    }
    Ok(out)
}

/// Load a single skill from a path.
pub fn load(path: &Path) -> Result<Skill> {
    let content = std::fs::read_to_string(path)?;
    parser::parse(&content, Some(path.to_path_buf()))
}

/// Install a skill from a local path into the local registry.
///
/// v0.1: copy SKILL.md to `root/<name>/<version>/SKILL.md`.
/// Future: support HTTP URLs, git refs, etc.
pub fn install_from_path(path: &Path) -> Result<PathBuf> {
    let skill = load(path)?;
    let fm = skill.frontmatter.ok_or_else(|| {
        SkillError::Validation("cannot install prose skill: no frontmatter".into())
    })?;

    let dest_dir = root()?.join(&fm.name).join(fm.version.to_string());
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join("SKILL.md");
    std::fs::copy(path, &dest)?;
    Ok(dest)
}

impl From<walkdir::Error> for SkillError {
    fn from(e: walkdir::Error) -> Self {
        SkillError::Other(e.to_string())
    }
}

//! Parse a SKILL.md (or soapstone-style markdown) into a `Skill`.

use crate::error::{Result, SkillError};
use crate::frontmatter::Frontmatter;
use crate::skill::{Skill, SkillBody, SkillKind};
use std::path::PathBuf;

/// Parse markdown content into a `Skill`. Detects frontmatter automatically.
pub fn parse(source: &str, path: Option<PathBuf>) -> Result<Skill> {
    let trimmed = source.trim_start_matches('\u{feff}');

    if trimmed.starts_with("---\n") || trimmed.starts_with("---\r\n") {
        parse_structured(trimmed, path)
    } else {
        // Prose mode — no frontmatter, free-form soapstone-style.
        Ok(Skill {
            kind: SkillKind::Prose,
            frontmatter: None,
            body: SkillBody {
                markdown: source.to_string(),
            },
            source_path: path,
        })
    }
}

fn parse_structured(source: &str, path: Option<PathBuf>) -> Result<Skill> {
    // Find the closing `---\n` after the opening one.
    let after_open = &source[source.find("---").unwrap() + 3..];
    let after_open = after_open.trim_start_matches(&['\r', '\n'][..]);

    let close_idx = find_fence_close(after_open).ok_or_else(|| {
        let where_ = path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "<stdin>".into());
        SkillError::NoFrontmatter(where_)
    })?;

    let (yaml_str, rest) = after_open.split_at(close_idx);
    let rest = rest
        .trim_start_matches("---")
        .trim_start_matches(&['\r', '\n'][..]);

    let frontmatter: Frontmatter =
        serde_yaml::from_str(yaml_str).map_err(|e| SkillError::Frontmatter(format!("{e}")))?;

    Ok(Skill {
        kind: SkillKind::Structured,
        frontmatter: Some(frontmatter),
        body: SkillBody {
            markdown: rest.to_string(),
        },
        source_path: path,
    })
}

fn find_fence_close(after_open: &str) -> Option<usize> {
    // Accept either "\n---" or a standalone line "---" possibly trailed by CR.
    let mut idx = 0;
    for line in after_open.split_inclusive('\n') {
        let stripped = line.trim_end_matches(&['\r', '\n'][..]);
        if stripped == "---" {
            return Some(idx);
        }
        idx += line.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prose_only() {
        let src = "# Finding UK train times\n\nCRS codes are...";
        let skill = parse(src, None).unwrap();
        assert!(matches!(skill.kind, SkillKind::Prose));
        assert!(skill.frontmatter.is_none());
    }

    #[test]
    fn parses_structured_with_frontmatter() {
        let src = "---\nname: pr-review\nversion: 0.1.0\ndescription: review a PR\n---\n\n# body\n";
        let skill = parse(src, None).unwrap();
        assert!(matches!(skill.kind, SkillKind::Structured));
        let fm = skill.frontmatter.unwrap();
        assert_eq!(fm.name, "pr-review");
        assert_eq!(fm.version.to_string(), "0.1.0");
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let src = "---\nname: broken\n\nbody with no close";
        assert!(parse(src, None).is_err());
    }
}

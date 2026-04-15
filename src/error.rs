use thiserror::Error;

pub type Result<T> = std::result::Result<T, SkillError>;

#[derive(Error, Debug)]
pub enum SkillError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid frontmatter: {0}")]
    Frontmatter(String),

    #[error("missing frontmatter delimiters in {0}")]
    NoFrontmatter(String),

    #[error("skill not found: {0}")]
    NotFound(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("skill execution failed: {0}")]
    ExecutionFailed(String),

    #[error("{0}")]
    Other(String),
}

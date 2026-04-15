//! `skill` - CLI for the skillrt runtime.
//!
//! Run `skill help` for command reference.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use skillrt::{parser, registry};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "skill")]
#[command(version, about = "Runtime for executable markdown skills", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a SKILL.md against the spec.
    Validate {
        /// Path to a SKILL.md file.
        path: PathBuf,
    },

    /// Show metadata for a SKILL.md (frontmatter parsed + body length).
    Info { path: PathBuf },

    /// List locally installed skills.
    List,

    /// Install a skill from a local path.
    Install { path: PathBuf },

    /// Run a skill (v0.1: prints the skill body; execution lands in v0.2).
    Run {
        /// Skill name or path to a SKILL.md.
        target: String,

        /// Inputs as JSON.
        #[arg(long, default_value = "{}")]
        inputs: String,
    },

    /// Print the path to the local skill store.
    Where,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => {
            let src = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let skill = parser::parse(&src, Some(path.clone()))?;
            match &skill.kind {
                skillrt::SkillKind::Prose => {
                    println!("ok (prose, {} bytes)", skill.body.markdown.len());
                }
                skillrt::SkillKind::Structured => {
                    let fm = skill.frontmatter.as_ref().unwrap();
                    println!("ok (structured) {}@{}", fm.name, fm.version);
                }
            }
        }

        Commands::Info { path } => {
            let src = std::fs::read_to_string(&path)?;
            let skill = parser::parse(&src, Some(path))?;
            match skill.kind {
                skillrt::SkillKind::Prose => {
                    println!("kind: prose");
                    println!("body-bytes: {}", skill.body.markdown.len());
                }
                skillrt::SkillKind::Structured => {
                    let fm = skill.frontmatter.unwrap();
                    println!("kind: structured");
                    println!("name: {}", fm.name);
                    println!("version: {}", fm.version);
                    println!("description: {}", fm.description);
                    println!("authors: {:?}", fm.authors);
                    println!("license: {}", fm.license);
                    println!("inputs: {}", fm.inputs.len());
                    println!("tags: {:?}", fm.tags);
                }
            }
        }

        Commands::List => {
            let skills = registry::list()?;
            if skills.is_empty() {
                println!("(no skills installed; try `skill install <path>`)");
            } else {
                for s in skills {
                    if let Some(fm) = s.frontmatter {
                        println!("{}@{}  {}", fm.name, fm.version, fm.description);
                    }
                }
            }
        }

        Commands::Install { path } => {
            let dest = registry::install_from_path(&path)?;
            println!("installed to {}", dest.display());
        }

        Commands::Run {
            target,
            inputs: _inputs,
        } => {
            let skill = if let Ok(p) = PathBuf::from(&target).canonicalize() {
                registry::load(&p)?
            } else {
                registry::list()?
                    .into_iter()
                    .find(|s| s.frontmatter.as_ref().is_some_and(|fm| fm.name == target))
                    .ok_or_else(|| anyhow::anyhow!("skill not found: {}", target))?
            };

            println!("{}", skill.body.markdown);
            eprintln!("(v0.1: printed body. Execution lands in v0.2.)");
        }

        Commands::Where => {
            println!("{}", registry::root()?.display());
        }
    }

    Ok(())
}

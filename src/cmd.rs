use std::path::PathBuf;

use clap::Parser;

/// The Command line arguments

#[derive(Debug, Parser)]
#[command(
    name = "minigrep",
    version,
    about = "search content recursively in the dirs"
)]
pub struct Args {
    #[arg(required_unless_present = "regexps", value_name = "PATTERN")]
    pub pattern: Option<String>,

    #[arg(short = 'e', long = "regexps", value_name = "PATTERN")]
    pub regexps: Vec<String>,

    #[arg(value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn all_patterns(&self) -> Vec<String> {
        let mut patterns = self.regexps.clone();
        if self.regexps.is_empty() {
            if let Some(pat) = &self.pattern {
                patterns.push(pat.clone());
            }
        }

        patterns
    }

    pub fn all_paths(&self) -> Vec<PathBuf> {
        let mut paths = self.paths.clone();
        if self.regexps.is_empty() {
            return self.paths.clone();
        }

        if let Some(p) = &self.pattern {
            paths.insert(0, PathBuf::from(p.clone()));
        }

        paths
    }
}

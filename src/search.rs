use std::{fs::File, io::Read, path::PathBuf};

use regex::bytes::{Regex, RegexBuilder};

use crate::{GrepResult, cmd::Args};

#[derive(Debug)]
pub struct FileResult {
    pub path: PathBuf,
    pub lines: Vec<Vec<u8>>,
    pub matched: Vec<usize>,
}

pub struct Matcher {
    pub regex: Regex,
}

impl Matcher {
    pub fn new(args: &Args) -> GrepResult<Self> {
        let patterns = args.all_patterns();
        let joined = patterns
            .iter()
            .map(|pat| format!("(?:{pat})"))
            .collect::<Vec<_>>()
            .join("|");

        let regex = RegexBuilder::new(&joined).build()?;
        Ok(Self { regex })
    }

    pub fn is_match(&self, line: &[u8]) -> bool {
        self.regex.is_match(line)
    }
}

pub fn split_lines(data: &[u8]) -> Vec<Vec<u8>> {
    data.split(|e| *e == '\n' as u8)
        .into_iter()
        .map(|d| d.to_vec())
        .collect()
}

pub fn search_file(path: &PathBuf, matcher: &Matcher) -> GrepResult<FileResult> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;

    let mut matched = Vec::new();
    let lines = split_lines(&data);
    for (index, line) in lines.iter().enumerate() {
        if matcher.is_match(line) {
            matched.push(index + 1);
        }
    }

    Ok(FileResult {
        path: path.clone(),
        lines,
        matched,
    })
}

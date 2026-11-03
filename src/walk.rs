use std::path::PathBuf;

use ignore::WalkBuilder;

use crate::GrepResult;

pub fn collect_files(paths: &[PathBuf]) -> GrepResult<Vec<PathBuf>> {
    let mut all_paths = paths.to_vec();
    if paths.is_empty() {
        all_paths.push(PathBuf::from("."));
    }

    let mut builder = WalkBuilder::new(&all_paths[0]);
    for p in &all_paths[1..] {
        builder.add(p);
    }

    builder.follow_links(false);

    let mut files = Vec::new();
    for entry in builder.build() {
        let entry = entry?;
        if entry.file_type().is_some_and(|f| f.is_file()) {
            files.push(entry.into_path());
        }
    }

    Ok(files)
}

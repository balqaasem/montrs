use railwind::{Source, parse_to_string};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::{Result, Context};
use std::fs;

pub fn compile(input_dir: &Path, output_file: &Path) -> Result<()> {
    let mut source = Source::new(railwind::CollectionOptions::new().html_compatible(true));

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs" || ext == "html") {
                let content = fs::read_to_string(path)?;
                source.append_string(&content);
            }
    }

    let css = parse_to_string(source, false, &[], &[]);
    
    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(output_file, css).context("Failed to write railwind CSS")?;
    Ok(())
}

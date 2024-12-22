mod digest;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use clap::Parser as _;
use walkdir::WalkDir;

use crate::digest::{sha256file, Sha256Value};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l', long)]
    left: Vec<PathBuf>,

    #[arg(short = 'r', long)]
    right: Vec<PathBuf>,
}

fn calculate_right_set(set: &mut HashSet<Sha256Value>, root: &Path) -> Result<()> {
    for entry in WalkDir::new(root).into_iter() {
        let entry = entry.context("Error walking directory")?;
        let path = entry.path();
        if path.is_file() {
            let digest = sha256file(path)?;
            set.insert(digest);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut right_set = HashSet::<Sha256Value>::new();
    for root in &args.right {
        calculate_right_set(&mut right_set, root)?;
    }

    for root in &args.left {
        for entry in WalkDir::new(root).into_iter() {
            let entry = entry.context("Error walking directory")?;
            let path = entry.path();
            if path.is_file() {
                let digest = sha256file(path)?;
                if !right_set.contains(&digest) {
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}

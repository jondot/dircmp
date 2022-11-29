#![allow(clippy::missing_const_for_fn)]
use regex::RegexSet;
use sha2::Digest;

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: `{0}`")]
    Io(#[from] io::Error),

    #[error("error: `{0}`")]
    Error(String),

    #[error("regex error: `{0}`")]
    Regex(#[from] regex::Error),

    #[error("path prefix error: `{0}`")]
    StripPrefix(#[from] std::path::StripPrefixError),
}

fn hash_file(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut hsh = sha2::Sha256::new();
    io::copy(&mut file, &mut hsh)?;
    Ok(format!("{:x}", hsh.finalize()))
}
#[derive(Debug, Default)]
pub struct Diff {
    pub right: PathBuf,
    pub left: PathBuf,
    pub similar: Vec<PathBuf>,
    pub changed: Vec<PathBuf>,
    pub missing_right: Vec<PathBuf>,
    pub missing_left: Vec<PathBuf>,
    pub different_type: Vec<PathBuf>,
}

impl Diff {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changed.is_empty()
            && self.missing_left.is_empty()
            && self.missing_right.is_empty()
            && self.different_type.is_empty()
    }
}

pub struct Comparison {
    excludes: RegexSet,
}

impl Comparison {
    #[must_use]
    pub fn new(excludes: RegexSet) -> Self {
        Self { excludes }
    }
}

impl Default for Comparison {
    fn default() -> Self {
        Self {
            excludes: RegexSet::empty(),
        }
    }
}

impl Comparison {
    fn walk(&self, path: &Path) -> impl Iterator<Item = DirEntry> + '_ {
        WalkDir::new(path).into_iter().flatten().filter(|entry| {
            !self
                .excludes
                .matches(&entry.path().display().to_string())
                .matched_any()
        })
    }

    ///
    /// Two way compare of left and right folders
    ///
    /// # Errors
    /// Returns error if IO fails
    pub fn compare<P: AsRef<Path>>(&self, left_folder: P, right_folder: P) -> Result<Diff, Error> {
        let left_folder = left_folder.as_ref();
        let right_folder = right_folder.as_ref();
        if !left_folder.exists() {
            return Err(Error::Error(format!(
                "folder: '{:?}' is missing",
                left_folder
            )));
        }
        if !right_folder.exists() {
            return Err(Error::Error(format!(
                "folder: '{:?}' is missing",
                right_folder
            )));
        }

        let mut diff_result = Diff {
            left: left_folder.to_path_buf(),
            right: right_folder.to_path_buf(),
            ..Default::default()
        };

        for left_entry in self.walk(left_folder) {
            let candidate = left_entry.path().strip_prefix(left_folder)?;
            let maybe_in_right = right_folder.join(candidate);
            if !maybe_in_right.exists() {
                diff_result
                    .missing_right
                    .push(left_entry.path().to_path_buf());
                continue;
            }

            // similar looking paths, check content -- or check type
            if left_entry.path().is_file() && maybe_in_right.is_file() {
                if hash_file(left_entry.path())? == hash_file(&maybe_in_right)? {
                    diff_result.similar.push(left_entry.into_path());
                } else {
                    diff_result.changed.push(left_entry.into_path());
                }
            } else if !(left_entry.path().is_dir() && maybe_in_right.is_dir()
                || left_entry.path().is_symlink() && maybe_in_right.is_symlink())
            {
                diff_result.different_type.push(left_entry.into_path());
            }
        }

        // complete the picture, swap the folders and check if anything on the right
        // is missing from the left. we don't need to address mutual stuff because that's
        // already done in the first loop
        for right_entry in self.walk(right_folder) {
            let candidate = right_entry.path().strip_prefix(right_folder)?;
            let maybe_in_left = left_folder.join(candidate);
            if !maybe_in_left.exists() {
                diff_result
                    .missing_left
                    .push(right_entry.path().to_path_buf());
                continue;
            }
        }

        Ok(diff_result)
    }
}

use fs3::FileExt;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use super::Metadata;

/// Represents an output directory on disk.
pub struct Output {
    path: PathBuf,
    index_file: File,
    all_metadata: HashMap<String, Metadata>,
}

impl Output {
    /// Creates a new output directory with the given path.
    pub fn new<P>(path: P) -> Output
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&path);
        let index_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path.join("Snapshots.toml"))
            .unwrap();

        index_file.lock_exclusive().unwrap();

        Output {
            path,
            index_file,
            all_metadata: HashMap::new(),
        }
    }

    /// Stores a snapshot with the given metadata in the output directory.
    pub fn store(&mut self, metadata: Metadata, screenshot: Vec<u8>) {
        let digest = metadata.digest().to_string();
        self.all_metadata.insert(digest.clone(), metadata);

        File::create(self.path.join(digest + ".png"))
            .unwrap()
            .write_all(&screenshot)
            .unwrap();
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.index_file.set_len(0).unwrap();
        self.index_file
            .write_all(
                toml::to_string_pretty(&self.all_metadata)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();

        self.index_file.unlock().unwrap();
    }
}

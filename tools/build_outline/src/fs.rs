
use std::{fs, path::Path};

pub fn read_dir(path: &Path) {
  fs::read_dir(path);
}
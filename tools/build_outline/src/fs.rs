use std::{
    fs::{self, ReadDir},
    io::Error,
    path::Path,
};

pub fn read_dir(path: &Path) -> Result<ReadDir, Error> {
    return fs::read_dir(path);
}

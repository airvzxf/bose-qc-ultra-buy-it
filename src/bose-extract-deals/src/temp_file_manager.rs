use crate::errors::AppError;
use std::fs;
use std::path::PathBuf;
use tempfile::{Builder, NamedTempFile};

pub struct TempFileManager {
    temp_dir: PathBuf,
    filename_prefix: String,
}

impl TempFileManager {
    pub fn new(temp_dir: &str, filename_prefix: &str) -> Result<Self, AppError> {
        let temp_dir = PathBuf::from(temp_dir);
        fs::create_dir_all(&temp_dir)?;

        Ok(TempFileManager {
            temp_dir,
            filename_prefix: filename_prefix.to_string(),
        })
    }

    pub fn create_file(&self) -> Result<NamedTempFile, AppError> {
        let temp_file = Builder::new()
            .prefix(&self.filename_prefix)
            .tempfile_in(&self.temp_dir)?;

        Ok(temp_file)
    }
}

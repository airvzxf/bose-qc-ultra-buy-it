use crate::errors::AppError;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn parse_db_path(db_raw_path: &String) -> Result<String, AppError> {
    let db_path = if !db_raw_path.is_empty() {
        PathBuf::from(db_raw_path)
    } else {
        home_dir()
            .ok_or_else(|| AppError::General("Unable to determine home directory".into()))?
            .join(".local")
            .join("share")
            .join("bose-extract-deals")
            .join("liverpool-bose-qc-ultra.db")
    };

    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::General(format!("Failed to create directory: {}", e)))?;
    }

    Ok(db_path.to_string_lossy().into_owned())
}

use crate::structures::Product;
use crate::AppError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn extract_json_data(html_content: &str) -> Result<String, AppError> {
    let start_pattern: &str =
        r#"<script id="__NEXT_DATA__" type="application/json" crossorigin="anonymous">"#;
    let end_pattern: &str = "</script>";

    let json_start_index: usize = html_content
        .find(start_pattern)
        .ok_or_else(|| AppError::General("Start pattern not found".to_string()))?;

    let json_content_start = json_start_index + start_pattern.len();
    let json_content_end = html_content[json_content_start..]
        .find(end_pattern)
        .ok_or_else(|| AppError::General("End pattern not found".to_string()))?;

    let json_data = &html_content[json_content_start..json_content_start + json_content_end];

    Ok(json_data.to_string())
}

pub fn store_json_to_file(json_data: &str, file_path: &Path) -> Result<(), AppError> {
    let mut file: File = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn get_file_content(file_path: &Path) -> Result<String, AppError> {
    let mut file: File = File::open(file_path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn store_product_to_json(product: &Product, file_path: &Path) -> Result<(), AppError> {
    let json_data: String = serde_json::to_string(product)?;

    store_json_to_file(&json_data, file_path)
}

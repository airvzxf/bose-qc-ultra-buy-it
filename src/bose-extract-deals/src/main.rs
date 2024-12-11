mod arguments;
mod database;
mod db_storage;
mod errors;
mod html_extractor;
mod json_extractor;
mod product_extractor;
mod structures;
mod temp_file_manager;

use arguments::Args;
use errors::AppError;
use rusqlite::Connection;
use structures::Product;
use temp_file_manager::TempFileManager;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    // Create a temporary file manager.
    let filename_path = "/tmp/";
    let filename_prefix = "liverpool-bose-qc-ultra-";
    let temp_file_manager = TempFileManager::new(filename_path, filename_prefix)?;

    // Create a temporary file for the HTML content.
    let html_filename = temp_file_manager.create_file()?;

    // Extract HTML content from the given URL.
    let html_content: String = html_extractor::extract_html_content(&args.url)?;
    html_extractor::store_response_to_file(&html_content, html_filename.path())?;

    // Create a temporary file for the JSON data.
    let json_data_filename = temp_file_manager.create_file()?;

    // Extract JSON data from the HTML content.
    let html_content: String = json_extractor::get_file_content(html_filename.path())?;
    let json_data: String = json_extractor::extract_json_data(&html_content)?;
    json_extractor::store_json_to_file(&json_data, json_data_filename.path())?;

    // Create a temporary file for the JSON summary.
    let json_summary_filename = temp_file_manager.create_file()?;

    // Extract JSON summary from the JSON data.
    let json_data: String = json_extractor::get_file_content(json_data_filename.path())?;
    let product: Product = product_extractor::extract_product(&json_data)?;
    json_extractor::store_product_to_json(&product, json_summary_filename.path())?;

    // Create sqlite database and insert product data.
    let db_path = db_storage::parse_db_path(&args.db_path)?;
    let conn: Connection = database::create_database(&db_path)?;
    database::insert_product(&conn, &product)?;

    Ok(())
}

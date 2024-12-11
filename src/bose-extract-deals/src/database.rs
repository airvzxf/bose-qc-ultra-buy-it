use crate::structures::Product;
use crate::AppError;
use rusqlite::params;
use rusqlite::{Connection, Result};

pub fn create_database(path: &str) -> Result<Connection, AppError> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            brand TEXT NOT NULL,
            color TEXT NOT NULL,
            time_recorded_utc TEXT NOT NULL,
            time_recorded_gmt_minus_6 TEXT NOT NULL,
            last_modified_time TEXT NOT NULL,
            creation_date TEXT NOT NULL,
            max_promo_price REAL NOT NULL,
            min_promo_price REAL NOT NULL,
            max_list_price REAL NOT NULL,
            min_list_price REAL NOT NULL,
            discount_percentage REAL NOT NULL,
            promo_price REAL NOT NULL,
            sale_price REAL NOT NULL,
            list_price REAL NOT NULL,
            sort_price REAL NOT NULL,
            last_modified_by_whom TEXT NOT NULL,
            rating_average REAL NOT NULL,
            rating_count INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS promotions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        products_id INTEGER NOT NULL,
        months INTEGER NOT NULL,
        promo_type TEXT NOT NULL,
        promo_desc TEXT NOT NULL,
        min_purchase_amount INTEGER NOT NULL,
        min_purchase_unit INTEGER NOT NULL,
        discount_unit INTEGER NOT NULL,
        discount_amount REAL NOT NULL,
        promo_code INTEGER NOT NULL,
        item_price REAL NOT NULL,
        monthly_price REAL NOT NULL,
        final_price REAL NOT NULL,
        final_price_distance REAL NOT NULL,
        different_price BOOLEAN NOT NULL,
        FOREIGN KEY(products_id) REFERENCES products(id)
    )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_products_product_id ON products (product_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_promotions_products_id ON promotions (products_id)",
        [],
    )?;

    // Create trigger if it doesn't exist

    Ok(conn)
}

pub fn insert_product(conn: &Connection, product: &Product) -> Result<(), AppError> {
    conn.execute(
        "INSERT OR REPLACE INTO products (product_id, title, brand, color, time_recorded_utc, time_recorded_gmt_minus_6, last_modified_time, creation_date, max_promo_price, min_promo_price, max_list_price, min_list_price, discount_percentage, promo_price, sale_price, list_price, sort_price, last_modified_by_whom, rating_average, rating_count) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
        params![
            product.product_id,
            product.title,
            product.brand,
            product.color,
            product.time_recorded_utc,
            product.time_recorded_gmt_minus_6,
            product.last_modified_time,
            product.creation_date,
            product.max_promo_price,
            product.min_promo_price,
            product.max_list_price,
            product.min_list_price,
            product.discount_percentage,
            product.promo_price,
            product.sale_price,
            product.list_price,
            product.sort_price,
            product.last_modified_by_whom,
            product.rating_average,
            product.rating_count,
        ],
    )?;

    let products_id: i64 = conn.last_insert_rowid();

    for promotion in &product.promotions {
        conn.execute(
            "INSERT OR IGNORE INTO promotions (
                products_id, months, promo_type, promo_desc, min_purchase_amount,
                min_purchase_unit, discount_unit, discount_amount, promo_code,
                item_price, monthly_price, final_price, final_price_distance, different_price
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                products_id,
                promotion.months,
                promotion.promo_type,
                promotion.promo_desc,
                promotion.min_purchase_amount,
                promotion.min_purchase_unit,
                promotion.discount_unit,
                promotion.discount_amount,
                promotion.promo_code,
                promotion.item_price,
                promotion.monthly_price,
                promotion.final_price,
                promotion.final_price_distance,
                promotion.different_price,
            ],
        )?;
    }

    Ok(())
}

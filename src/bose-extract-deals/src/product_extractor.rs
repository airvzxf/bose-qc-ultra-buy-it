use crate::structures::Promotion;
use crate::AppError;
use crate::Product;
use chrono::{DateTime, FixedOffset, Utc};
use serde_json::{Map, Value};
use std::str::FromStr;

fn extract_value<T>(data: &Map<String, Value>, key: &str) -> Result<T, AppError>
where
    T: serde::de::DeserializeOwned,
{
    data.get(key)
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .ok_or_else(|| AppError::General(format!("Failed to extract {}", key)))
}

fn convert_to_numeric<T>(value: &str) -> Result<T, AppError>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|e| AppError::General(format!("Failed to convert '{}' to numeric: {}", value, e)))
}

pub fn extract_product(json_data: &str) -> Result<Product, AppError> {
    let json: Value = serde_json::from_str(json_data)
        .map_err(|e| AppError::General(format!("Failed to parse JSON: {}", e)))?;

    // Extract the mainContent from the JSON data.
    let main_content: &Map<String, Value> = json["query"]["data"]["mainContent"]
        .as_object()
        .ok_or_else(|| AppError::General("Failed to extract mainContent".to_string()))?;

    // Extract records from the mainContent.
    let records: &Vec<Value> = main_content["records"]
        .as_array()
        .ok_or_else(|| AppError::General("Failed to extract records".to_string()))?;

    // Extract allMeta from the first record.
    let all_meta: &Map<String, Value> = records
        .first()
        .and_then(|record| record["allMeta"].as_object())
        .ok_or_else(|| AppError::General("Failed to extract allMeta".to_string()))?;

    // Extract ratingInfo from allMeta.
    let rating_info: &Map<String, Value> = all_meta["ratingInfo"]
        .as_object()
        .ok_or_else(|| AppError::General("Failed to extract ratingInfo".to_string()))?;

    // Extract variants from allMeta.
    let variants: &Vec<Value> = all_meta["variants"]
        .as_array()
        .ok_or_else(|| AppError::General("Failed to extract variants".to_string()))?;

    // Extract first variant from variants.
    let first_variant: &Map<String, Value> = variants
        .first()
        .and_then(|v| v.as_object())
        .ok_or_else(|| AppError::General("Failed to extract first variant".to_string()))?;

    // Extract prices from first variant.
    let prices: &Map<String, Value> = first_variant["prices"]
        .as_object()
        .ok_or_else(|| AppError::General("Failed to extract prices".to_string()))?;

    // Extract liverpoolPromotionsEMI from first variant.
    let liverpool_promotions_emi: &Vec<Value> = first_variant["liverpoolPromotionsEMI"]
        .as_array()
        .ok_or_else(|| AppError::General("Failed to extract prices".to_string()))?;

    let promotions: Vec<Promotion> = liverpool_promotions_emi
        .iter()
        .filter_map(|promo| {
            let mut promo_emi: Promotion = match serde_json::from_value(promo.clone()) {
                Ok(emi) => emi,
                Err(_) => return None,
            };

            let months = promo_emi.months as f64;
            let monthly_price = promo_emi.monthly_price;
            let item_price = promo_emi.item_price;
            let final_price: f64 = months * monthly_price;
            let price_distance = (item_price - final_price).abs();
            promo_emi.final_price = final_price;
            promo_emi.final_price_distance = price_distance;
            promo_emi.different_price = price_distance > 9.0;

            Some(promo_emi)
        })
        .collect();

    let product_id: String = extract_value(main_content, "productId")?;
    let product_id: usize = convert_to_numeric(&product_id)?;
    let title: String = extract_value(all_meta, "title")?;
    let brand: String = extract_value(all_meta, "brand")?;
    let last_modified_time: String = extract_value(all_meta, "lastmodifiedTime")?;
    let creation_date: String = extract_value(all_meta, "creationDate")?;
    let max_promo_price: f64 = extract_value(all_meta, "maximumPromoPrice")?;
    let min_promo_price: f64 = extract_value(all_meta, "minimumPromoPrice")?;
    let max_list_price: f64 = extract_value(all_meta, "maximumListPrice")?;
    let min_list_price: f64 = extract_value(all_meta, "minimumListPrice")?;
    let last_modified_by_whom: String = extract_value(all_meta, "lastmodifiedByWhom")?;
    let rating_average: String = extract_value(rating_info, "productAvgRating")?;
    let rating_average: f64 = convert_to_numeric(&rating_average)?;
    let rating_count: String = extract_value(rating_info, "productRatingCount")?;
    let rating_count: usize = convert_to_numeric(&rating_count)?;
    let color: String = extract_value(first_variant, "color")?;
    let discount_percentage: String = extract_value(prices, "discountPercentage")?;
    let discount_percentage: f64 = convert_to_numeric(&discount_percentage)?;
    let promo_price: String = extract_value(prices, "promoPrice")?;
    let promo_price: f64 = convert_to_numeric(&promo_price)?;
    let sale_price: String = extract_value(prices, "salePrice")?;
    let sale_price: f64 = convert_to_numeric(&sale_price)?;
    let list_price: String = extract_value(prices, "listPrice")?;
    let list_price: f64 = convert_to_numeric(&list_price)?;
    let sort_price: String = extract_value(prices, "sortPrice")?;
    let sort_price: f64 = convert_to_numeric(&sort_price)?;

    let now_utc: DateTime<Utc> = Utc::now();
    let now_gmt_minus_6: DateTime<FixedOffset> = now_utc.with_timezone(
        &FixedOffset::west_opt(6 * 3600)
            .ok_or_else(|| AppError::General("Invalid timezone offset".to_string()))?,
    );

    let time_recorded_utc: String = now_utc.to_rfc3339();
    let time_recorded_gmt_minus_6: String = now_gmt_minus_6.to_rfc3339();

    Ok(Product {
        product_id,
        title,
        brand,
        color,
        time_recorded_utc,
        time_recorded_gmt_minus_6,
        last_modified_time,
        creation_date,
        max_promo_price,
        min_promo_price,
        max_list_price,
        min_list_price,
        discount_percentage,
        promo_price,
        sale_price,
        list_price,
        sort_price,
        last_modified_by_whom,
        rating_average,
        rating_count,
        promotions,
    })
}

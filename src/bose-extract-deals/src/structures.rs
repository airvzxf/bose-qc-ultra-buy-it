use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_id: usize,
    pub title: String,
    pub brand: String,
    pub color: String,
    pub time_recorded_utc: String,
    pub time_recorded_gmt_minus_6: String,
    pub last_modified_time: String,
    pub creation_date: String,
    pub max_promo_price: f64,
    pub min_promo_price: f64,
    pub max_list_price: f64,
    pub min_list_price: f64,
    pub discount_percentage: f64,
    pub promo_price: f64,
    pub sale_price: f64,
    pub list_price: f64,
    pub sort_price: f64,
    pub last_modified_by_whom: String,
    pub rating_average: f64,
    pub rating_count: usize,
    pub promotions: Vec<Promotion>,
}

#[derive(Debug, Serialize)]
pub struct Promotion {
    pub months: usize,
    #[serde(rename = "promoType")]
    pub promo_type: String,
    #[serde(rename = "promoDesc")]
    pub promo_desc: String,
    #[serde(rename = "minPurchaseAmount")]
    pub min_purchase_amount: usize,
    #[serde(rename = "minPurchaseUnit")]
    pub min_purchase_unit: usize,
    #[serde(rename = "discountUnit")]
    pub discount_unit: usize,
    #[serde(rename = "discountAmount")]
    pub discount_amount: f64,
    #[serde(rename = "promoCode")]
    pub promo_code: usize,
    #[serde(rename = "itemPrice")]
    pub item_price: f64,
    #[serde(rename = "monthlyPrice")]
    pub monthly_price: f64,
    #[serde(default)]
    pub final_price: f64,
    #[serde(default)]
    pub final_price_distance: f64,
    #[serde(default)]
    pub different_price: bool,
}

impl<'de> Deserialize<'de> for Promotion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct RawPromotions {
            months: String,
            #[serde(rename = "promoType")]
            promo_type: String,
            #[serde(rename = "promoDesc")]
            promo_desc: String,
            #[serde(rename = "minPurchaseAmount")]
            min_purchase_amount: String,
            #[serde(rename = "minPurchaseUnit")]
            min_purchase_unit: String,
            #[serde(rename = "discountUnit")]
            discount_unit: String,
            #[serde(rename = "discountAmount")]
            discount_amount: String,
            #[serde(rename = "promoCode")]
            promo_code: String,
            #[serde(rename = "itemPrice")]
            item_price: String,
            #[serde(rename = "monthlyPrice")]
            monthly_price: String,
        }

        let raw: RawPromotions = RawPromotions::deserialize(deserializer)?;

        let months: usize = usize::from_str(&raw.months).map_err(serde::de::Error::custom)?;
        let min_purchase_amount =
            usize::from_str(&raw.min_purchase_amount).map_err(serde::de::Error::custom)?;
        let min_purchase_unit =
            usize::from_str(&raw.min_purchase_unit).map_err(serde::de::Error::custom)?;
        let discount_unit =
            usize::from_str(&raw.discount_unit).map_err(serde::de::Error::custom)?;
        let discount_amount =
            f64::from_str(&raw.discount_amount).map_err(serde::de::Error::custom)?;
        let promo_code = usize::from_str(&raw.promo_code).map_err(serde::de::Error::custom)?;
        let item_price = f64::from_str(&raw.item_price).map_err(serde::de::Error::custom)?;
        let monthly_price = f64::from_str(&raw.monthly_price).map_err(serde::de::Error::custom)?;

        Ok(Promotion {
            months,
            promo_type: raw.promo_type,
            promo_desc: raw.promo_desc,
            min_purchase_amount,
            min_purchase_unit,
            discount_unit,
            discount_amount,
            promo_code,
            item_price,
            monthly_price,
            final_price: 0.0,
            final_price_distance: 0.0,
            different_price: false,
        })
    }
}

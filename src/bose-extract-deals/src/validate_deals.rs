use crate::structures::Product;

pub fn review_product_promotions(product: &Product) -> Vec<&str> {
    let mut results: Vec<&str> = Vec::new();

    for promotion in &product.promotions {
        if promotion.months > 9 && promotion.promo_type == "PD" {
            results.push("Promotion with more than 9 months and type PD found");
        }

        if promotion.discount_amount > 10.0 {
            results.push("Promotion with discount amount greater than 10% found");
        }
    }

    results
}

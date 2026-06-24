use crate::products::models::Product;

pub fn extract_sku(no_marks: &str, active_products: &[Product]) -> Option<String> {
    for p in active_products {
        let sku_lower = p.sku.to_lowercase();
        let sku_clean = sku_lower.replace(" ", "");
        if no_marks.contains(&sku_clean) {
            return Some(p.sku.clone());
        }
    }
    None
}

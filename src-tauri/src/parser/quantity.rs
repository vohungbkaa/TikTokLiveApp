use regex::Regex;

pub fn extract_quantity(text: &str) -> i32 {
    let re = Regex::new(r"(?i)(?:x\s*)?(\d+)(?:\s*(?:cai|chiec|bo|set))?").unwrap();
    for cap in re.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            if let Ok(qty) = m.as_str().parse::<i32>() {
                if qty > 0 {
                    return qty;
                }
            }
        }
    }
    1
}

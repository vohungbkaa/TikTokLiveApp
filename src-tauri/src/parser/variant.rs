pub fn extract_variants(no_marks: &str) -> Vec<String> {
    let mut variants = Vec::new();
    
    let sizes = ["xxl", "xl", "size s", "size m", "size l", " s ", " m ", " l "];
    for s in sizes.iter() {
        if no_marks.contains(s) {
            variants.push(s.trim().to_uppercase());
        }
    }
    
    let colors = ["do", "den", "trang", "xanh", "vang", "hong"];
    for c in colors.iter() {
        if no_marks.contains(c) {
            variants.push(c.to_string());
        }
    }
    
    variants
}

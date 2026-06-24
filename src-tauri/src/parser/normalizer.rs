pub fn normalize(text: &str) -> String {
    let lower = text.to_lowercase();
    let trimmed = lower.trim();
    // simple remove diacritics using a basic map for Vietnamese
    let mut no_diacritics = String::with_capacity(trimmed.len());
    for c in trimmed.chars() {
        let replacement = match c {
            'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => "a",
            'đ' => "d",
            'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => "e",
            'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => "i",
            'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => "o",
            'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => "u",
            'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => "y",
            _ => { no_diacritics.push(c); continue; }
        };
        no_diacritics.push_str(replacement);
    }
    
    // remove emoji and control chars
    no_diacritics.retain(|c| c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_');
    
    no_diacritics
}

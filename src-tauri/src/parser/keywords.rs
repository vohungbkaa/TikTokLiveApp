use super::models::IntentType;

pub fn determine_intent(_text: &str, no_marks: &str) -> (IntentType, Vec<String>) {
    let mut rules = Vec::new();
    let mut intent = IntentType::Noise;

    let buy_keywords = ["chot", "lay", "mua", "cho minh", "don"];
    let cancel_keywords = ["huy", "thoi", "doi"];
    let question_keywords = ["gia", "con khong", "con ko", "bao nhieu", "size"];

    for kw in cancel_keywords.iter() {
        if no_marks.contains(kw) {
            intent = IntentType::Cancel;
            rules.push(format!("cancel_kw:{}", kw));
            return (intent, rules);
        }
    }

    for kw in question_keywords.iter() {
        if no_marks.contains(kw) {
            intent = IntentType::Question;
            rules.push(format!("question_kw:{}", kw));
            return (intent, rules);
        }
    }

    for kw in buy_keywords.iter() {
        if no_marks.contains(kw) {
            intent = IntentType::Buy;
            rules.push(format!("buy_kw:{}", kw));
            break;
        }
    }

    (intent, rules)
}

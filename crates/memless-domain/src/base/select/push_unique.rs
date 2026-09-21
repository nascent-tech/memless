pub(crate) fn push_unique(seen: &mut Vec<String>, key: &str) {
    if !seen.iter().any(|name| name == key) {
        seen.push(key.to_string());
    }
}

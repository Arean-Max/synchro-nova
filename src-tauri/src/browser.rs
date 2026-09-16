pub(crate) fn open_driver_search_url(query: &str) -> Result<(), String> {
    let query = clean_query(query);
    if query.is_empty() {
        return Err("Empty driver search query".to_string());
    }
    open_url(&format!(
        "https://www.google.com/search?q={}",
        percent_encode(&query)
    ))
}

fn clean_query(query: &str) -> String {
    query
        .chars()
        .filter(|ch| !ch.is_control())
        .take(180)
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => {
                encoded.push(byte as char)
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

fn open_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        return Err("Invalid or insecure URL scheme".to_string());
    }
    crate::ffi::open_path_or_url(url).map_err(|_| "Failed to open browser".to_string())
}

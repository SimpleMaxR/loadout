use serde_json::Value;

/// Redact sensitive fields (tokens, API keys, passwords) from MCP config
/// before displaying in UI
pub fn redact(config: &Value) -> Value {
    match config {
        Value::Object(map) => {
            let mut redacted = serde_json::Map::new();
            for (k, v) in map {
                if is_sensitive_key(k) {
                    redacted.insert(k.clone(), Value::String("***".to_string()));
                } else {
                    redacted.insert(k.clone(), redact(v));
                }
            }
            Value::Object(redacted)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(redact).collect()),
        other => other.clone(),
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_lowercase();
    lower.contains("token")
        || lower.contains("secret")
        || lower.contains("password")
        || lower.contains("api_key")
        || lower.contains("apikey")
        || lower.contains("auth")
        || lower.contains("credential")
        || lower.contains("bearer")
}

use tracing_subscriber::{EnvFilter, fmt};

/// Initialize structured logging with secret-safe defaults.
/// Reads `RUST_LOG` (default: `rusteams=info`).
pub fn init() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("rusteams=info"));
    let _ = fmt().with_env_filter(filter).with_target(false).try_init();
}

/// Redact `Authorization` header values and bearer tokens from a log line.
pub fn redact(line: &str) -> String {
    let mut out = line.to_string();
    for secret in ["Bearer ", "bearer "] {
        let replacement = format!("{secret}[REDACTED]");
        let mut search_from = 0;
        while let Some(rel) = out[search_from..].find(secret) {
            let i = search_from + rel;
            let token_start = i + secret.len();
            let token_end = out[token_start..]
                .find([' ', '\n', '"'])
                .map(|e| token_start + e)
                .unwrap_or(out.len());
            out.replace_range(i..token_end, &replacement);
            search_from = i + replacement.len();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_tokens_are_redacted() {
        let line = redact("GET /me/chats Authorization: Bearer eyJhbGciOiJIUzI1NiJ9 done");
        assert!(!line.contains("eyJ"));
        assert!(line.contains("[REDACTED]"));
    }
}

//! Utilities for redacting sensitive values from log strings.

use regex::Regex;
use std::sync::OnceLock;

// We compile regexes lazily and cache them with OnceLock to avoid the runtime
// cost of recompiling on every log call while staying thread-safe.

fn email_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\b[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}\b")
            .expect("email regex should compile")
    })
}

fn vin_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\b[A-HJ-NPR-Z0-9]{17}\b").expect("vin regex should compile"))
}

fn bearer_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\b(Bearer)\s+[A-Za-z0-9._\-~+/]+=*").expect("bearer regex should compile")
    })
}

fn password_quoted_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)("?(?:password|pass|pwd|pf\.pass)"?\s*[:=]\s*")([^"]+)(")"#)
            .expect("quoted password regex should compile")
    })
}

fn password_plain_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)(\b(?:password|pass|pwd|pf\.pass)\b\s*[:=]\s*)([^\s,;\"]+)"#)
            .expect("plain password regex should compile")
    })
}

fn token_quoted_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?i)("?(?:access_token|refresh_token|access-token|refresh-token)"?\s*[:=]\s*")([^"]+)(")"#,
        )
        .expect("quoted token regex should compile")
    })
}

fn token_plain_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r#"(?i)(\b(?:access_token|refresh_token|access-token|refresh-token)\b\s*[:=]\s*)([^\s,;\"]+)"#,
        )
        .expect("plain token regex should compile")
    })
}

/// Redact sensitive values in a string so it can be safely logged.
///
/// This masks:
/// - Email addresses
/// - Password fields (`password`, `pass`, `pwd`, `pf.pass`)
/// - VINs
/// - Access/refresh tokens and bearer tokens
pub fn redact_str(input: &str) -> String {
    let mut out = input.to_owned();

    out = password_quoted_re()
        .replace_all(&out, "$1[REDACTED_PASSWORD]$3")
        .into_owned();
    out = password_plain_re()
        .replace_all(&out, "$1[REDACTED_PASSWORD]")
        .into_owned();

    out = token_quoted_re()
        .replace_all(&out, "$1[REDACTED_TOKEN]$3")
        .into_owned();
    out = token_plain_re()
        .replace_all(&out, "$1[REDACTED_TOKEN]")
        .into_owned();
    out = bearer_re()
        .replace_all(&out, "$1 [REDACTED_TOKEN]")
        .into_owned();

    out = email_re()
        .replace_all(&out, "[REDACTED_EMAIL]")
        .into_owned();
    out = vin_re().replace_all(&out, "[REDACTED_VIN]").into_owned();

    out
}

#[cfg(test)]
mod tests {
    use super::redact_str;

    #[test]
    fn redacts_email() {
        let input = "user=[example_user]";
        let output = redact_str(input);
        assert_eq!(output, "user=[REDACTED_EMAIL]");
    }

    #[test]
    fn redacts_passwords() {
        let input = r#"password=secret pass:topsecret "pf.pass":"abc123""#;
        let output = redact_str(input);
        assert!(output.contains("password=[REDACTED_PASSWORD]"));
        assert!(output.contains("pass:[REDACTED_PASSWORD]"));
        assert!(output.contains(r#""pf.pass":"[REDACTED_PASSWORD]""#));
    }

    #[test]
    fn redacts_vin() {
        let input = "vin=ABCD1234ABCDEFGH1";
        let output = redact_str(input);
        assert_eq!(output, "vin=[REDACTED_VIN]");
    }

    #[test]
    fn redacts_tokens() {
        let input = r#"access_token=abc123 refresh_token:"xyz987" Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.payload.sig"#;
        let output = redact_str(input);
        assert!(output.contains("access_token=[REDACTED_TOKEN]"));
        assert!(output.contains(r#"refresh_token:"[REDACTED_TOKEN]""#));
        assert!(output.contains("Bearer [REDACTED_TOKEN]"));
    }

    #[test]
    fn redacts_mixed_message() {
        let input = "Login failed for account@example.invalid vin=ABCD1234ABCDEFGH1 password=top_secret";
        let output = redact_str(input);
        assert_eq!(
            output,
            "Login failed for [REDACTED_EMAIL] vin=[REDACTED_VIN] password=[REDACTED_PASSWORD]"
        );
    }
}

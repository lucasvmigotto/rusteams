// Shared HTTPS client: built-in WebPKI roots plus an optional extra CA bundle.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! All outbound HTTPS (Graph, Entra) goes through [`build_client`], so TLS
//! trust is configured in exactly one place. Besides the platform WebPKI
//! roots, `RUSTEAMS_CA_BUNDLE` points at a PEM file with extra CA
//! certificates (self-signed simulation CAs, corporate proxies). The path
//! — never key material — is the only thing that reaches error messages.

use crate::error::AppError;

/// Env var holding the path to an extra PEM CA bundle.
pub const CA_BUNDLE_ENV: &str = "RUSTEAMS_CA_BUNDLE";

/// Build the shared HTTPS client. Fails closed on unreadable/invalid bundles.
pub fn build_client() -> Result<reqwest::Client, AppError> {
    let mut builder = reqwest::Client::builder().tls_built_in_webpki_certs(true);
    if let Ok(path) = std::env::var(CA_BUNDLE_ENV) {
        let pem = std::fs::read(&path)
            .map_err(|e| AppError::Config(format!("read CA bundle {path}: {e}")))?;
        let certs = reqwest::Certificate::from_pem_bundle(&pem)
            .map_err(|_| AppError::Config(format!("parse CA bundle {path}")))?;
        if certs.is_empty() {
            return Err(AppError::Config(format!("no certificates in CA bundle {path}")));
        }
        for cert in certs {
            builder = builder.add_root_certificate(cert);
        }
    }
    builder.build().map_err(|e| AppError::Config(format!("build HTTPS client: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Throwaway self-signed CA for tests only (public certificate, no key).
    const TEST_CA_PEM: &str = "-----BEGIN CERTIFICATE-----\n\
        MIIDITCCAgmgAwIBAgIUYnmiFy6o7NacOSNnJO5uxsXvBHYwDQYJKoZIhvcNAQEL\n\
        BQAwIDEeMBwGA1UEAwwVcnVzdGVhbXMtaHR0cC10ZXN0LWNhMB4XDTI2MDkyNjE3\n\
        MTc1NVoXDTI2MDkyODE3MTc1NVowIDEeMBwGA1UEAwwVcnVzdGVhbXMtaHR0cC10\n\
        ZXN0LWNhMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqn3rnZ8v9UaZ\n\
        +wGZJ9aWjU8DyBcCKPLTdL0zOkmTwAST66fkcYt8II45W3Jjnj52wE4cFLthFv+V\n\
        vJ/+F2ayK8nU8ZiNJ6kXMj+OJ/sMAPi1g+d8TTq/HUc4MD5oCrOy5XYsI2Nz/fUL\n\
        WBhD61fbkwyvPbeSSJhy8j8ACY+Pk2K1rBhbKBbMY0IUvEWs1mFxRrbSyVtE8KQ/\n\
        ySXi5+qXpveLNjB6H35d1Nqt06BlyEGR+2By23IA5o9hVosQz7w+03q3LzaQ4svj\n\
        oOdCyDutpBdDjQ2KMPCnRq3h6rF80phArKM7WoDKo67JK4qrzSoLQYhv3QXFrnuD\n\
        GUQdspF9BwIDAQABo1MwUTAdBgNVHQ4EFgQUDLfF1kQvrNEfvRSNJxlViZ52vPow\n\
        HwYDVR0jBBgwFoAUDLfF1kQvrNEfvRSNJxlViZ52vPowDwYDVR0TAQH/BAUwAwEB\n\
        /zANBgkqhkiG9w0BAQsFAAOCAQEACpyXmPY1FOUULEYvZfytgjWVFgnVcFj1g2t4\n\
        5IQ2ZXuzfgvt3UxDiI+zlLMv2Gq2C0JXRf6U6O08aFRPTTxsdXe5+tgq2/Q6aQeB\n\
        AnS1CLNWx1H38eNefOqfUJEM2svutzVE2Mlw5n6Vwh2lp/TI22a0Bvah1WHfMdt6\n\
        kxtdBX50ElXAWbUi/rY8VbfPJmc90sPFLgw4s2dWYN50fVgwsyqL33LhZO1QLLwy\n\
        t2bmVvzo/X+LFVxXhWT7wAS0pk75Oxvvpscsi9lwmM3a3Q3nkqZzpwNJ1vn9MtXO\n\
        PbCOZLEq7lTsHgAJy1mogQmB+VN/yrLtXzOuwHFBKbdpkFS8aw==\n\
        -----END CERTIFICATE-----\n";

    static ENV_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn builds_with_no_bundle_configured() {
        let _guard = env_guard();
        unsafe { std::env::remove_var(CA_BUNDLE_ENV) };
        assert!(build_client().is_ok());
    }

    #[test]
    fn loads_a_valid_pem_bundle() {
        let _guard = env_guard();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ca.pem");
        std::fs::write(&path, TEST_CA_PEM).unwrap();
        unsafe { std::env::set_var(CA_BUNDLE_ENV, &path) };
        assert!(build_client().is_ok());
        unsafe { std::env::remove_var(CA_BUNDLE_ENV) };
    }

    #[test]
    fn missing_bundle_file_fails_closed() {
        let _guard = env_guard();
        unsafe { std::env::set_var(CA_BUNDLE_ENV, "/nonexistent/rusteams-ca.pem") };
        assert!(build_client().is_err());
        unsafe { std::env::remove_var(CA_BUNDLE_ENV) };
    }

    #[test]
    fn malformed_bundle_fails_closed_without_leaking_contents() {
        let _guard = env_guard();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ca.pem");
        std::fs::write(&path, "not-a-pem-bundle").unwrap();
        unsafe { std::env::set_var(CA_BUNDLE_ENV, &path) };
        let err = build_client().unwrap_err();
        assert!(!err.user_message().contains("not-a-pem-bundle"));
        unsafe { std::env::remove_var(CA_BUNDLE_ENV) };
    }
}

// OS-backed refresh-token storage. Access tokens stay in memory only.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::error::AppError;

/// Abstraction so tests never touch the real OS keyring.
pub trait SecretStore: Send + Sync {
    fn save_refresh_token(&self, account: &str, token: &str) -> Result<(), AppError>;
    fn load_refresh_token(&self, account: &str) -> Result<Option<String>, AppError>;
    fn clear_refresh_token(&self, account: &str) -> Result<(), AppError>;
}

/// Production keyring-backed store.
pub struct KeyringStore {
    service: String,
}

impl KeyringStore {
    pub fn new(service: &str) -> Self {
        Self { service: service.into() }
    }
}

impl SecretStore for KeyringStore {
    fn save_refresh_token(&self, account: &str, token: &str) -> Result<(), AppError> {
        keyring::Entry::new(&self.service, account)
            .and_then(|e| e.set_password(token))
            .map_err(|_| AppError::Security("credential store unavailable".into()))
    }

    fn load_refresh_token(&self, account: &str) -> Result<Option<String>, AppError> {
        match keyring::Entry::new(&self.service, account) {
            Ok(e) => match e.get_password() {
                Ok(pw) => Ok(Some(pw)),
                Err(keyring::Error::NoEntry) => Ok(None),
                Err(_) => Err(AppError::Security("credential store unavailable".into())),
            },
            Err(_) => Err(AppError::Security("credential store unavailable".into())),
        }
    }

    fn clear_refresh_token(&self, account: &str) -> Result<(), AppError> {
        match keyring::Entry::new(&self.service, account) {
            Ok(e) => match e.delete_credential() {
                Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                Err(_) => Err(AppError::Security("credential store unavailable".into())),
            },
            Err(_) => Err(AppError::Security("credential store unavailable".into())),
        }
    }
}

/// In-memory store for tests and `--no-keyring` escape hatch.
#[derive(Default)]
pub struct MemoryStore {
    inner: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

impl SecretStore for MemoryStore {
    fn save_refresh_token(&self, account: &str, token: &str) -> Result<(), AppError> {
        self.inner.lock().unwrap().insert(account.into(), token.into());
        Ok(())
    }
    fn load_refresh_token(&self, account: &str) -> Result<Option<String>, AppError> {
        Ok(self.inner.lock().unwrap().get(account).cloned())
    }
    fn clear_refresh_token(&self, account: &str) -> Result<(), AppError> {
        self.inner.lock().unwrap().remove(account);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_roundtrip_and_clear() {
        let s = MemoryStore::default();
        assert_eq!(s.load_refresh_token("a").unwrap(), None);
        s.save_refresh_token("a", "rt-123").unwrap();
        assert_eq!(s.load_refresh_token("a").unwrap().as_deref(), Some("rt-123"));
        s.clear_refresh_token("a").unwrap();
        assert_eq!(s.load_refresh_token("a").unwrap(), None);
    }
}

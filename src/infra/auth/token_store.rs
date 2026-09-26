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

/// File-backed store for headless/simulation use, selected via the
/// `RUSTEAMS_TOKEN_FILE` env var (see CLI). JSON object `{account: token}`,
/// created owner-readable-only on unix. The path is configuration; file
/// contents never reach logs or error messages.
pub struct FileStore {
    path: std::path::PathBuf,
}

/// Env var selecting [`FileStore`] over the OS keyring.
pub const TOKEN_FILE_ENV: &str = "RUSTEAMS_TOKEN_FILE";

impl FileStore {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn read_all(&self) -> Result<std::collections::HashMap<String, String>, AppError> {
        match std::fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|_| AppError::Security("token file unreadable".into())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Default::default()),
            Err(_) => Err(AppError::Security("token file unreadable".into())),
        }
    }

    fn write_all(&self, map: &std::collections::HashMap<String, String>) -> Result<(), AppError> {
        let bytes = serde_json::to_vec(map)
            .map_err(|_| AppError::Security("token file unwritable".into()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&self.path)
                .and_then(|mut f| {
                    use std::io::Write;
                    f.write_all(&bytes)
                })
                .map_err(|_| AppError::Security("token file unwritable".into()))
        }
        #[cfg(not(unix))]
        {
            std::fs::write(&self.path, bytes)
                .map_err(|_| AppError::Security("token file unwritable".into()))
        }
    }
}

impl SecretStore for FileStore {
    fn save_refresh_token(&self, account: &str, token: &str) -> Result<(), AppError> {
        let mut map = self.read_all()?;
        map.insert(account.into(), token.into());
        self.write_all(&map)
    }
    fn load_refresh_token(&self, account: &str) -> Result<Option<String>, AppError> {
        Ok(self.read_all()?.get(account).cloned())
    }
    fn clear_refresh_token(&self, account: &str) -> Result<(), AppError> {
        let mut map = self.read_all()?;
        map.remove(account);
        self.write_all(&map)
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

    #[test]
    fn file_store_roundtrip_clear_and_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        let s = FileStore::new(&path);
        assert_eq!(s.load_refresh_token("default").unwrap(), None);
        s.save_refresh_token("default", "rt-456").unwrap();
        assert_eq!(s.load_refresh_token("default").unwrap().as_deref(), Some("rt-456"));
        s.clear_refresh_token("default").unwrap();
        assert_eq!(s.load_refresh_token("default").unwrap(), None);
    }

    #[test]
    fn file_store_rejects_corrupt_contents_without_leaking() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        std::fs::write(&path, "{not json").unwrap();
        let err = FileStore::new(&path).load_refresh_token("default").unwrap_err();
        assert!(!err.user_message().contains("not json"));
    }

    #[cfg(unix)]
    #[test]
    fn file_store_is_owner_readable_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        let s = FileStore::new(&path);
        s.save_refresh_token("default", "rt-789").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}

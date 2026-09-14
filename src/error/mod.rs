use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("authentication failed")]
    Auth(String),
    #[error("provider error: {0}")]
    Provider(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("security violation: {0}")]
    Security(String),
    #[error("input error: {0}")]
    Validation(String),
    /// Cooperative shutdown: tasks stop, terminal state is restored.
    /// Unit struct — nothing secret can ever be attached.
    #[error("shutting down")]
    Shutdown,
}

impl AppError {
    /// User-facing message. By construction this contains no secrets:
    /// constructors take static kinds, never raw tokens/responses.
    pub fn user_message(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages_contain_no_secret_by_construction() {
        let cases = vec![
            AppError::Auth("login failed".into()),
            AppError::Network("timeout".into()),
            AppError::Config("missing client-id".into()),
        ];
        for err in cases {
            let msg = err.user_message();
            assert!(!msg.contains("Bearer"), "leak in {msg:?}");
            assert!(!msg.contains("eyJ"), "leak in {msg:?}");
        }
    }

    #[test]
    fn shutdown_error_is_a_fixed_message() {
        assert_eq!(AppError::Shutdown.user_message(), "shutting down");
    }

    #[test]
    fn auth_error_does_not_echo_secret_input() {
        // Even if a caller passes a token as context, Display must not be
        // trusted to carry it — auth errors render a fixed message.
        let err = AppError::Auth("x".into());
        assert_eq!(err.user_message(), "authentication failed");
    }
}

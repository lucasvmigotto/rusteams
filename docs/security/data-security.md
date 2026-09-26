# Data security

In-memory message state only (no persistent cache in MVP). Keyring holds refresh
tokens; file perms restrictive for config (`~/.config/rusteams/`); logs redact
Bearer material; crash output must not include message bodies. Locally stored:
non-secret config only. `rusteams logout` clears credentials.

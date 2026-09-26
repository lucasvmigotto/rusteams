# Authentication

BYO Entra public-client app, device code flow, `organizations` authority default.
Scopes (least privilege): `User.Read Chat.Read ChatMessage.Send Chat.ReadWrite
Presence.Read Presence.Read.All offline_access openid profile`.
Personal accounts unsupported by Teams chat APIs — fail closed with guidance.
Refresh token in OS keyring (`rusteams` service), access tokens memory-only,
logout clears. No secrets in config files, logs, or error displays.

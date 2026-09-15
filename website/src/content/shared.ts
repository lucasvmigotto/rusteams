export const SCOPES = [
  "openid",
  "profile",
  "offline_access",
  "User.Read",
  "Chat.Read",
  "ChatMessage.Send",
  "Chat.ReadWrite",
  "Presence.Read",
  "Presence.Read.All",
] as const;

export const ENV_VARS = [
  ["RUSTEAMS_CLIENT_ID", "Entra public-client application (client) ID. Required for login."],
  ["RUSTEAMS_TENANT_ID", "Entra tenant. Default: organizations."],
  [
    "RUSTEAMS_GRAPH_BASE_URL",
    "Microsoft Graph base URL. Default: https://graph.microsoft.com/v1.0.",
  ],
  ["RUSTEAMS_POLL_INTERVAL_SECS", "Polling interval in seconds. Default: 15, minimum: 1."],
  ["RUST_LOG", "Log filter. Default: rusteams=info."],
] as const;

export const CLI_COMMANDS = [
  ["rusteams", "Launch the TUI. Configure first: rusteams config."],
  ["rusteams login", "Device-code login. Requires a client ID (RUSTEAMS_CLIENT_ID)."],
  ["rusteams logout", "Clear the stored refresh token."],
  ["rusteams status", "Show logged_in and tenant_id. Never prints secrets."],
  ["rusteams config", "Show tenant_id, graph_base_url, poll_interval_secs, client_id set/unset."],
  ["rusteams doctor", "Show config file path and keyring reachability."],
  ["rusteams version", "Print rusteams <version>."],
] as const;

export const KEYBINDINGS = [
  ["j / Ctrl+N", "Next chat", "Implemented"],
  ["k / Ctrl+P", "Previous chat", "Implemented"],
  ["i", "Enter compose mode", "Implemented (live loop)"],
  ["Enter (compose)", "Submit draft", "Implemented (live loop)"],
  ["Esc (compose)", "Abandon draft", "Implemented (live loop)"],
  ["Backspace (compose)", "Delete character", "Implemented (live loop)"],
  ["Enter", "Open chat", "Bound, no-op (planned)"],
  ["/ or Ctrl+K", "Command palette", "Bound, no-op (planned)"],
  ["Ctrl+Q", "Quit", "Implemented"],
] as const;

export const GRAPH_ENDPOINTS = [
  ["GET /me/chats", "List chats (paged, lastMessagePreview expanded)."],
  ["GET /me/chats/{id}/messages", "List messages (paged, ordered)."],
  ["GET …/messages?$filter=lastModifiedDateTime gt …", "Incremental poll since a watermark."],
  ["GET /me/chats/{id}/messages/{mid}", "Hydrate a single message (search hits)."],
  ["POST /me/chats/{id}/messages", "Send (text, mention HTML, file reference, quote reply)."],
  ["PATCH …/messages/{mid}", "Edit a message (re-GET confirms)."],
  ["POST …/messages/{mid}/softDelete", "Soft-delete a message."],
  ["POST …/messages/{mid}/setReaction", "Set a reaction."],
  ["POST …/messages/{mid}/unsetReaction", "Remove a reaction."],
  ["GET …/messages/{mid}/hostedContents", "List hosted content (5 MiB fetch cap)."],
  ["POST /search/query", "Search chatMessage entities."],
  ["GET /me/presence", "Read own presence."],
] as const;

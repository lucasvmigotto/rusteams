# Entra ID app setup (BYO-app model) and live login verification

## Prerequisites

- A **work or school** Microsoft 365 account. Personal Microsoft accounts are
  NOT supported by the Teams chat APIs — login will fail closed with guidance.
- Access to the [Entra admin center](https://entra.microsoft.com) (or ask your
  tenant admin) to register an app and grant consent.

## 1. Register a public-client app

1. Entra admin center → Identity → Applications → App registrations → New.
2. Name: `rusteams` (or anything). Supported account types: single- or
   multi-tenant **work/school only**.
3. Redirect URI: public client / mobile + desktop,
   `https://login.microsoftonline.com/common/oauth2/nativeclient`.
4. Authentication → Advanced → **Allow public client flows: Yes**.
5. API permissions → Add **delegated** Microsoft Graph permissions (least
   privilege, consent each):
   `openid profile offline_access User.Read Chat.Read ChatMessage.Send
   Chat.ReadWrite Presence.Read Presence.Read.All`.
6. Note the **Application (client) ID** and your **tenant ID** (or use
   `organizations` for multi-tenant).

## 2. Configure rusteams

```bash
export RUSTEAMS_CLIENT_ID="<application-client-id>"
export RUSTEAMS_TENANT_ID="organizations"   # or your tenant id
rusteams config   # never prints secrets
```

## 3. Log in and verify live

```bash
rusteams login    # prints the device code + verification URL; approve in browser
rusteams status   # logged_in: true (refresh token lives in the OS keyring)
rusteams doctor   # keyring reachable: true
rusteams logout   # clears the keyring entry
```

## 4. Troubleshooting

- `client-id is not set` → export `RUSTEAMS_CLIENT_ID` first.
- `authentication failed` during polling → code expired (>15 min), declined, or
  tenant blocks device flow; re-run `login`.
- Conditional Access / MFA is handled in the browser step, not in the TUI.
- No tokens are ever written to disk outside the OS keyring; `status`/`doctor`
  never print secrets. Report suspected leaks per `SECURITY.md`, not in public
  issues.

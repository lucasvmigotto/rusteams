import type { DocsContent } from "./types";

export const enDocs: DocsContent = {
  overview: {
    tagline: "Teams for the terminal",
    intro:
      "rusteams is a terminal user interface (TUI) client for Microsoft Teams via Microsoft Graph. These docs describe the project's actual state at the current version — what works, what is partial, and what is still planned.",
    mvpNote: "MVP core — core ready, live loop pending",
    maturity: {
      title: "Maturity notice",
      body: "Live chat listing, message send/receive, full TUI, and realtime polling are not ready for daily use yet. Anything claiming otherwise is a documentation bug.",
    },
    whatWorksTitle: "What works today",
    cardCliBody: "login · logout · status · config · doctor · version",
    cardAuthTitle: "Authentication",
    cardAuthBody: "Device-code OAuth2 with your own Entra app, silent refresh, OS keyring.",
    cardGraphTitle: "Graph client",
    cardGraphBody:
      "Chats, messages, presence, search, edits, reactions, quotes, mentions, file references — with paging, retry, and sanitization.",
    cardTuiBody:
      "Read panes, composer, visual notification queue, tested keymap. Live loop and real-TTY verification pending.",
    quickstartTitle: "Quick start",
    prereqsTitle: "Prerequisites",
    prereqsBody:
      "Rust 1.89.0, Linux x86_64/ARM64, a work/school account (personal accounts unsupported), and your own Entra app. Secrets never go into config files.",
  },
  gettingStarted: {
    intro:
      "Installation, authentication, configuration, and first run — only what actually exists in the code.",
    installationTitle: "Installation",
    installationBody:
      "Supported platforms: Linux x86_64/ARM64 with Rust 1.89.0. Images published as bare X.Y.Z only, no latest.",
    authenticationTitle: "Authentication",
    authenticationBody:
      "BYO model: you register your own public Entra app (device-code flow, RFC 8628). Work/school accounts only; personal accounts fail closed.",
    noSecrets: {
      title: "No secrets",
      body: "Never put secrets in examples, logs, or files. Access tokens live in memory only; only the refresh token goes to the keyring.",
    },
    configurationTitle: "Configuration",
    realBehavior: {
      title: "Actual behavior today",
      body: "The code-documented precedence (CLI > env > file > defaults) is not wired into run() yet: only defaults + env take effect. The TOML file exists but nothing reads it; --client-id appears in an error string but is not a real flag. Use environment variables.",
    },
    firstRunTitle: "First run",
    firstRunBody:
      "Without a session the app guides you: no session; run rusteams login. Keys: j/k navigate, i compose, Enter send, Esc abandon, Ctrl+Q quit.",
  },
  usage: {
    intro: "TUI overview, real shortcuts, and notifications — with no-ops clearly marked.",
    tuiBodyA:
      "Header (title + connection), sidebar (30%), conversation (70%), and a status bar with hints + version. Empty states show hints, never panic. Connection: Connected, Degraded, Disconnected, Reconnecting, or Syncing.",
    tuiBodyB:
      "Messages render styled segments: underlined links with visible URL, bold indented code blocks, plain-text fallback. All remote text passes the sanitizer (C0/CSI/OSC/controls stripped).",
    shortcutsTitle: "Keyboard shortcuts",
    boundNote: {
      title: "Bound ≠ working",
      body: "Enter (open) and / or Ctrl+K (palette) are mapped but no-ops in the current fold — documented as planned. Compose mode (i, Enter, Esc, Backspace) works in the live loop.",
    },
    notificationsBody:
      "Bounded visual queue with 30s TTL, sanitized text, per-chat unread flags. Policy: visual-only — no bell by default; opt-in bell is explicit future work, non-MVP.",
  },
  architecture: {
    intro:
      "Layers, providers, Graph, sync, and persistence — how the system is actually put together.",
    layersTitle: "Layers",
    layersBody:
      "The TUI never calls Graph directly. Pure reducers (transition/apply) with caller-owned clocks/RNG; cooperative cancellation via Shutdown. In-memory cache only — no persistence until a new ADR.",
    graphTitle: "Microsoft Graph",
    graphBody:
      "Shared retry discipline: 429/5xx with capped Retry-After, 401 fast-fail. HTML becomes RichSegments (links, code, lists) with text fallback.",
    syncTitle: "Sync",
    syncBody:
      "Polling-first, no webhooks in the MVP: poll + diff + reconnect state machine with watermarks, backoff, and missed-event healing. Honest polling latency — never claim push/realtime.",
    persistenceTitle: "Persistence",
    persistenceBody:
      "Memory only. Refresh token in the OS keyring (rusteams/default); access tokens never leave memory; logout clears. Logs redact Bearer tokens.",
  },
  development: {
    intro: "Setup, testing, CI/CD, and contributing — the repository's real flow.",
    setupBody:
      "Docker DevContainer development; copy .env.example to .env (RUST_LOG — the code reads RUST_LOG, not LOG_LEVEL).",
    testingBody:
      "Phase-gated TDD: failing test → minimal implementation → style → docs. Offline suites (mock + wiremock + proptest) plus manual runbooks for live TTY, real tenant, and screen reader — live verification never runs in CI.",
    cicdBody:
      "ci.yml: fmt, check/clippy/test/build on x86_64 + ARM64. security.yml: cargo audit + cargo deny check (weekly). release.yml: quality → multi-arch build → dual-registry image (X.Y.Z, no latest) → annotated tag, never moved.",
    contributingBody:
      "See CONTRIBUTING.md: feat/* branches, small frequent commits, never invent behavior — derive everything from code and label implemented vs. planned.",
  },
  cli: {
    intro: "All commands, as implemented in src/cli.",
    commandsTitle: "Commands",
    flagsTitle: "Flags",
    missingFlags: {
      title: "Flags that do not exist",
      body: "--client-id, --graph-base-url, --poll-interval, and --no-keyring do not exist. The login error suggests --client-id, but it is only a string — use RUSTEAMS_CLIENT_ID.",
    },
    examplesTitle: "Examples",
  },
  configuration: {
    intro: "AppConfig shape, all variables, and the real precedence.",
    shapeTitle: "Shape",
    envTitle: "Environment variables",
    realPrecedence: {
      title: "Real precedence",
      body: "Effective today: defaults + env. The TOML file and CLI overrides exist in code but run() does not apply them.",
    },
  },
  keybindings: {
    intro: "Complete mapping from src/tui/keys.rs — unlisted keys are ignored, never panic.",
    allTitle: "All keys",
  },
  security: {
    intro: "How rusteams protects credentials, renders untrusted text, and audits dependencies.",
    credentialsTitle: "Credentials",
    credentialsBody:
      "Bring your own Entra app: the client ID is public configuration, not a secret. Access tokens stay in memory only; the refresh token lives in the OS keyring (service rusteams, account default) and logout clears it. Least-privilege delegated scopes; personal accounts fail closed.",
    terminalTitle: "Terminal rendering",
    terminalBody:
      "Every remote string passes sanitize() before reaching ratatui: C0 controls, CSI/OSC sequences, DEL, Fe/charset codes, and non-characters are stripped. Raw body.content is never rendered; there are no clickable remote hyperlinks — URLs print as visible text.",
    supplyChainTitle: "Supply chain",
    supplyChainBody:
      "cargo audit + cargo deny check run in CI and weekly (GPL-3.0-compatible licenses only, locked registry). Release images are distroless nonroot, versioned X.Y.Z with no latest tag.",
  },
  roadmap: {
    intro:
      "Where the project stands and what comes next. Status labels match the repository phases.",
    doneTitle: "Done (MVP core)",
    doneBody:
      "CLI, device-code auth with silent refresh, Graph chat/message/presence/search/ops client, pure reducers, polling engine, read panes, composer, visual notifications.",
    nextTitle: "Next (needs live verification)",
    nextBody:
      "Live tenant login and Graph calls, live TTY event loop, timer-driven sync binding, screen-reader and full keyboard-only passes, tenant smoke tests.",
    futureTitle: "Future (strategic)",
    futureBody:
      "Teams/channels, meetings and calls, OneDrive byte upload, transcripts, bots, admin surfaces, themes. Each needs API, permission, and feasibility review — no implementation before phases 0–6 land.",
  },
  faq: {
    intro: "Short answers to common questions.",
    items: [
      {
        question: "Do I need my own Entra app?",
        answer:
          "Yes. rusteams ships no client ID; register a public client app with redirect https://login.microsoftonline.com/common/oauth2/nativeclient and export RUSTEAMS_CLIENT_ID.",
      },
      {
        question: "Can I use a personal Microsoft account?",
        answer:
          "No. Work/school accounts only — personal accounts fail closed during login. This is an Entra policy limitation, not a bug.",
      },
      {
        question: "Where do I put my client ID?",
        answer:
          "In the RUSTEAMS_CLIENT_ID environment variable. The TOML config file parses but is not read by the binary yet, and --client-id is not a real flag.",
      },
      {
        question: "Is there a notification sound?",
        answer:
          "No, by design. Notifications are visual-only; an opt-in bell behind a config flag is documented future work.",
      },
    ],
  },
  troubleshooting: {
    intro: "Fix the most common failures with the exact strings the CLI prints.",
    items: [
      {
        title: "client-id is not set",
        body: "Set RUSTEAMS_CLIENT_ID (or RUSTEAMS_TENANT_ID=organizations). There is no --client-id flag despite what the error suggests.",
      },
      {
        title: "no session; run rusteams login",
        body: "No refresh token in the keyring. Run rusteams login, then rusteams status to confirm logged_in: true.",
      },
      {
        title: "credential store unavailable",
        body: "The OS keyring is unreachable (common in headless Docker). Tokens stay memory-only for that run; re-authenticate each launch.",
      },
      {
        title: "authentication failed / graph rejected credentials",
        body: "Refresh or access token rejected. Re-run login; for personal accounts see the FAQ — they are unsupported.",
      },
    ],
  },
  limitations: {
    intro: "What cannot exist in a terminal client, and why.",
    graphTitle: "Microsoft Graph limitations",
    graphBody:
      "List responses carry no read state, so unread badges come from local viewpoint only. Hosted-content fetch caps at 5 MiB. File byte upload is OneDrive-API territory — only pre-uploaded file references are supported.",
    tuiTitle: "Terminal limitations",
    tuiBody:
      "No voice/video calls, no meeting canvas, no rich cards or theming — a terminal renders sanitized text. Open and palette keys are bound but not yet functional; compose, navigation, and quit are fully wired.",
  },
};

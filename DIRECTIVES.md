# RUSTEAMS — MASTER ENGINEERING / ARCHITECTURE / IMPLEMENTATION PROMPT

You are the **principal software architect, senior Rust engineer, senior full-stack engineer, security engineer, TUI/UX engineer, QA engineer, DevOps engineer, and technical documentation lead** responsible for designing and implementing an open-source project named:

# `rusteams`

`rusteams` is a **terminal user interface (TUI) client for Microsoft Teams**.

The long-term objective is to provide the functionality that is realistically exposed and usable through Microsoft Graph and that can be meaningfully represented in a terminal interface.

The MVP is deliberately focused on **real-time chat**, but the architecture must avoid becoming an architectural dead end for future Teams functionality.

The project must be developed as a serious, enterprise-grade open-source project with a strong emphasis on:

1. Security
2. Correctness
3. Maintainability
4. Testability
5. Performance
6. Excellent terminal UX
7. Cross-platform portability
8. Clean architecture
9. Long-term extensibility
10. High-quality documentation

The project is licensed under **GNU GPL v3**.

---

# 1. ABSOLUTE ENGINEERING PRINCIPLES

These principles are mandatory.

## 1.1 Do not guess

Do not invent:

* Microsoft API capabilities
* Microsoft Graph endpoints
* authentication behavior
* realtime capabilities
* API semantics
* undocumented protocol behavior
* security guarantees
* platform capabilities
* dependency behavior
* terminal behavior
* feature requirements
* architectural constraints

When a requirement is ambiguous but can be resolved through authoritative technical documentation, research it.

When it cannot be resolved confidently, **STOP and ask the user**.

Never silently turn an assumption into a requirement.

Maintain an explicit list of unresolved questions/decisions when necessary.

---

# 2. AUTONOMOUS EXECUTION

Operate autonomously.

Do not ask for permission for every normal engineering operation.

You may autonomously:

* inspect the repository
* create directories
* create files
* modify source code
* add tests
* refactor
* run tests
* run linters
* run formatters
* analyze dependencies
* create documentation
* create CI configuration
* create development tooling
* investigate APIs
* improve architecture
* fix defects
* improve test coverage
* update phase documentation

However:

## AUTONOMY MUST NEVER MEAN GUESSING

Stop and ask the user when a decision is genuinely ambiguous and cannot be resolved through:

1. Existing project requirements
2. Existing project documentation
3. Existing source code
4. Authoritative Microsoft documentation
5. Rust documentation
6. Relevant standards/specifications
7. Existing architectural decisions documented in the project

Do not ask questions whose answers can reasonably be determined by engineering judgment.

---

# 3. FIRST TASK: DISCOVERY BEFORE IMPLEMENTATION

Do NOT immediately start coding.

First perform a comprehensive project bootstrap and architectural discovery.

Determine:

* current repository state
* whether this is an empty repository
* existing files
* existing Cargo configuration
* existing documentation
* existing CI
* existing source
* existing tests
* existing licenses
* existing conventions
* repository metadata
* Git configuration
* build environment
* Rust toolchain
* available development tools

If the repository is empty, initialize the project properly.

Before implementation, produce a concise but comprehensive internal assessment covering:

* project goals
* constraints
* known requirements
* explicit non-requirements
* technical unknowns
* API constraints
* security constraints
* platform constraints
* architecture candidates
* risks
* dependency risks
* testing strategy
* documentation strategy

Do not implement major functionality before completing this analysis.

---

# 4. MICROSOFT TEAMS INTEGRATION STRATEGY

Use the following strategy:

## Official Microsoft APIs + Provider Abstraction

The project must use **official Microsoft Graph APIs** wherever possible.

Do NOT use:

* reverse-engineered Teams protocols
* browser scraping
* unofficial Teams clients
* undocumented private APIs
* credential extraction from the official Teams client
* browser automation as the primary integration mechanism

The architecture must introduce a provider abstraction so that the application core is not tightly coupled to Microsoft Graph.

Conceptually:

```text
                     ┌─────────────────────┐
                     │       rusteams       │
                     │       TUI            │
                     └──────────┬──────────┘
                                │
                     ┌──────────▼──────────┐
                     │ Application / Core  │
                     │ Domain + Use Cases  │
                     └──────────┬──────────┘
                                │
                  ┌─────────────┼─────────────┐
                  │             │             │
           ┌──────▼──────┐ ┌────▼─────┐ ┌────▼──────┐
           │ Chat Service │ │ Auth     │ │ Event     │
           │              │ │ Service  │ │ Service   │
           └──────┬───────┘ └────┬─────┘ └────┬──────┘
                  │              │             │
                  └──────────────┼─────────────┘
                                 │
                       ┌─────────▼─────────┐
                       │ Provider Boundary │
                       └─────────┬─────────┘
                                 │
                       ┌─────────▼─────────┐
                       │ Microsoft Graph   │
                       └───────────────────┘
```

This is illustrative only.

You MUST evaluate this architecture and alternatives before implementation.

Do not blindly implement the diagram.

The final architecture must be justified in project documentation.

---

# 5. AUTHENTICATION

Use a terminal-friendly modern authentication strategy.

Prefer:

* Microsoft Entra ID
* OAuth 2.0
* Device Code Flow where appropriate for CLI/TUI environments
* Authorization Code + PKCE where appropriate
* secure token lifecycle management

Determine the exact Microsoft-recommended approach based on the current Microsoft documentation and the capabilities required by the MVP.

Requirements:

* never store plaintext secrets
* never log credentials
* never log access tokens
* never log refresh tokens
* minimize credential lifetime
* securely handle expiration
* securely refresh tokens where supported
* fail safely when authentication expires
* support enterprise tenants
* clearly document authentication prerequisites

Investigate whether personal Microsoft accounts are supported for the intended API functionality.

Do not promise unsupported account types.

---

# 6. MVP FUNCTIONAL SCOPE

The MVP is **real-time Microsoft Teams chat**.

The following are REQUIRED MVP capabilities.

## Conversations

* 1:1 conversations
* group chats
* conversation list/sidebar
* open conversation
* message history
* conversation search

## Messages

* send messages
* receive messages in real time
* edit messages
* delete messages
* reply/quote
* reactions
* mentions
* rich text / Markdown-compatible representation where technically appropriate
* attachments
* files
* emojis
* message search
* read receipts
* message threading

## User/communication state

* presence
* notifications
* connection state
* reconnect behavior
* automatic synchronization after reconnection

## Explicitly NOT MVP

Do not implement these unless required as a technical dependency:

* pagination as a user-visible feature
* typing indicators
* offline mode
* offline reading
* offline drafts
* queued messages
* draft synchronization

If Microsoft APIs require pagination internally to retrieve complete data, you may implement pagination internally. This does NOT mean pagination becomes a user-facing MVP feature.

---

# 7. REAL-TIME REQUIREMENT

Real-time message reception is a core MVP requirement.

Do not fake real-time behavior with arbitrary polling unless Microsoft Graph's officially supported mechanisms make that unavoidable.

Investigate the officially supported Microsoft mechanisms for:

* change notifications
* subscriptions
* event delivery
* message changes
* chat changes
* reconnect behavior
* subscription expiration
* renewal
* event ordering
* duplicate events
* missed events
* eventual consistency

Determine the most appropriate architecture.

The event system must account for:

* reconnects
* transient failures
* duplicate events
* out-of-order events
* missed notifications
* expired subscriptions
* API throttling
* network interruptions
* application restarts

After reconnecting, the application should automatically synchronize the state required to restore consistency.

Do not claim stronger consistency guarantees than Microsoft Graph provides.

---

# 8. LOCAL CACHE / PERSISTENCE

The MVP does NOT support offline mode.

Do not introduce local persistence merely because it is convenient.

However, cached message data may be implemented **if and only if it provides a measurable or clearly justified benefit**, such as:

* reducing API requests
* improving perceived performance
* improving UI responsiveness
* supporting reconnection synchronization
* reducing repeated retrieval of unchanged data

Any cache must follow strict security rules.

## Never persist sensitive information unnecessarily

Do not persist:

* access tokens in plaintext
* refresh tokens in plaintext
* credentials
* secrets
* private keys
* authentication cookies
* sensitive diagnostics
* unnecessary personal data

If authentication credentials must be persisted, use an appropriate OS-backed secure credential mechanism.

Message caching must be carefully evaluated because Teams messages are potentially sensitive enterprise information.

Before implementing persistent message caching:

1. Analyze the security/privacy implications.
2. Determine whether persistence is genuinely useful.
3. Define retention behavior.
4. Define invalidation behavior.
5. Define data minimization rules.
6. Document the decision.

If a cache is implemented, provide a mechanism to clear it.

Prefer ephemeral/in-memory state when persistent caching is not justified.

---

# 9. RECONNECTION

The application must recover gracefully from transient failures.

Implement a robust connection lifecycle.

Conceptually:

```text
CONNECTED
   │
   ▼
DEGRADED
   │
   ▼
DISCONNECTED
   │
   ▼
RECONNECTING
   │
   ├───────────────┐
   ▼               │
SYNCING            │
   │               │
   ▼               │
CONNECTED ◄────────┘
```

Evaluate the exact state machine.

Account for:

* exponential backoff
* jitter
* rate limiting
* subscription renewal
* authentication expiration
* partial synchronization
* duplicate events
* network recovery

Do not create reconnect loops that can overload the service.

---

# 10. TUI TECHNOLOGY

The application MUST be implemented in:

* Rust
* `ratatui`

The TUI must be designed as a proper application, not as a collection of ad-hoc widgets.

Separate:

* rendering
* event handling
* application state
* domain state
* asynchronous operations
* network operations
* provider implementation

The TUI must remain responsive while network operations are occurring.

Never block the rendering/event loop on network I/O.

---

# 11. TUI MVP DESIGN

Use the following UX direction as the baseline:

```text
┌──────────────────────────────────────────────────────────────────────┐
│ rusteams                                                ● Connected │
├───────────────┬──────────────────────────────────────────────────────┤
│ Chats         │ # Alice                                             │
│               │                                                      │
│ ● Alice       │ Alice                         10:32                  │
│   Bob         │ Hey, are you available?                             │
│   Engineering │                                                      │
│   Project X    │ You                           10:33                  │
│               │ Yes, I'm here.                                      │
│               │                                                      │
│               │                                                      │
├───────────────┴──────────────────────────────────────────────────────┤
│ > Type a message...                                      [Enter] Send │
├──────────────────────────────────────────────────────────────────────┤
│ Ctrl+K Search   Ctrl+N New Chat   Ctrl+R Refresh   Ctrl+Q Quit       │
└──────────────────────────────────────────────────────────────────────┘
```

This is a UX direction, not a pixel-perfect requirement.

The final design must be aesthetically coherent.

Required:

* chat list/sidebar
* conversation view
* message composer
* status bar
* command palette
* keyboard shortcuts
* minimal modal dialogs
* searchable conversations
* notifications
* connection status
* error/status notifications
* accessibility considerations
* Vim-style navigation
* Emacs-style navigation

Mouse support:

* implement if it can be done cleanly
* do not compromise architecture or keyboard UX merely to support it
* mouse support is optional

---

# 12. TUI ARCHITECTURE

The TUI must not directly call Microsoft Graph.

Prefer a structure conceptually similar to:

```text
UI
 │
 ▼
Application State
 │
 ▼
Commands / Actions
 │
 ▼
Use Cases
 │
 ▼
Domain Services
 │
 ▼
Provider Interfaces
 │
 ▼
Microsoft Graph Adapter
```

Evaluate whether this is the best architecture.

The final architecture should enable:

* deterministic UI testing
* mocked provider testing
* domain-level testing
* integration testing
* future providers
* future Teams capabilities
* independent UI evolution

---

# 13. KEYBOARD UX

Provide a coherent keyboard system.

At minimum establish shortcuts for:

* navigation
* selecting conversations
* opening search
* sending messages
* editing messages
* deleting messages
* replying
* reactions
* opening command palette
* refreshing
* reconnecting
* opening help
* quitting

Support both:

## Vim-style navigation

Examples may include:

* `j/k`
* `h/l`
* `/`
* `gg`
* `G`
* `Ctrl+d`
* `Ctrl+u`

## Emacs-style navigation

Where appropriate:

* `Ctrl+n`
* `Ctrl+p`
* `Ctrl+f`
* `Ctrl+b`
* `Ctrl+a`
* `Ctrl+e`

Do not overload keys unnecessarily.

Document all shortcuts.

Allow future configurable keybindings without requiring an architectural rewrite.

---

# 14. ACCESSIBILITY

Treat terminal accessibility as a real engineering requirement.

Consider:

* color-independent meaning
* sufficient visual distinction
* textual status indicators
* screen-reader-friendly output where terminal capabilities permit
* predictable focus behavior
* keyboard-only operation
* no information conveyed exclusively through color
* configurable display density
* Unicode fallback behavior
* narrow-terminal behavior
* large message handling
* terminal resize behavior

Do not claim full accessibility compliance unless it is demonstrably supported.

---

# 15. THEMES

Configurable themes are NOT an MVP priority.

Do not spend significant implementation effort on them during early phases.

However, design the rendering system so themes can be introduced later without rewriting the UI.

At the very end of the initial roadmap, create a detailed specification for future configurable themes.

---

# 16. SECURITY — FIRST-CLASS REQUIREMENT

Security is one of the project's highest priorities.

Perform a formal threat analysis.

At minimum consider:

## Authentication

* OAuth security
* token storage
* token leakage
* token expiration
* refresh token security
* authentication redirects
* device-code security

## Network

* TLS validation
* certificate validation
* HTTPS-only communication
* proxy behavior
* DNS risks
* connection hijacking
* API endpoint validation

## Application

* malicious message contents
* malformed API responses
* untrusted attachments
* untrusted filenames
* malformed Unicode
* terminal escape sequences
* ANSI injection
* OSC sequences
* control characters
* terminal title manipulation
* hyperlink abuse
* clipboard-related attacks

## TUI-specific security

This is especially important.

A malicious Teams message must NEVER be able to execute terminal control sequences.

Sanitize untrusted text before rendering.

Treat all remote data as hostile input.

Never allow message content to manipulate:

* terminal colors outside intended rendering
* cursor position
* terminal title
* terminal modes
* hyperlinks in unsafe ways
* clipboard
* keyboard input
* shell commands

Do not use unsafe terminal escape handling merely for aesthetics.

## Local data

Consider:

* cache confidentiality
* permissions
* local filesystem attacks
* symlink attacks
* configuration poisoning
* log leakage
* crash dumps
* environment variables
* temporary files

## Dependencies

Establish supply-chain security practices.

Consider:

* `cargo audit`
* `cargo deny`
* dependency pinning/lockfile
* license auditing
* vulnerability monitoring
* malicious dependency risks
* transitive dependencies

---

# 17. SECURITY DOCUMENTATION

Create dedicated security documentation.

At minimum:

```text
docs/security/
├── threat-model.md
├── authentication.md
├── data-security.md
├── terminal-security.md
├── dependency-security.md
└── incident-response.md
```

Adjust this structure if a better organization is justified.

Create:

* threat model
* trust boundaries
* attack surfaces
* security assumptions
* mitigations
* residual risks
* security testing strategy

Also create:

```text
SECURITY.md
```

with a responsible disclosure policy.

---

# 18. TDD IS MANDATORY

Develop using strict Test-Driven Development.

The default workflow is:

```text
RED
 ↓
Write failing test
 ↓
GREEN
 ↓
Minimal implementation
 ↓
REFACTOR
 ↓
Integration test
 ↓
Security/performance validation
```

Do NOT:

1. Implement a feature completely.
2. Add superficial tests afterward.

Tests must drive implementation.

---

# 19. TESTING STRATEGY

Use multiple test layers.

## Unit tests

Test:

* domain logic
* state machines
* reducers
* message transformations
* command handling
* validation
* parsing
* formatting
* synchronization logic
* retry/backoff logic
* cache behavior
* security sanitization

## Integration tests

Test complete application subsystems using mocks.

Examples:

```text
TUI
 ↓
Application
 ↓
Use Case
 ↓
Mock Provider
```

## Contract tests

Define provider contracts so alternative provider implementations can be validated consistently.

## TUI tests

Where appropriate use:

* deterministic rendering tests
* snapshot tests
* state transition tests
* keyboard interaction tests
* resize tests

## Property-based testing

Use it where it provides meaningful value, particularly for:

* parsing
* sanitization
* state transitions
* message ordering
* synchronization
* malformed input

## Security tests

Explicitly test:

* ANSI injection
* terminal control sequences
* malformed Unicode
* malicious filenames
* malicious API responses
* authentication failures
* token leakage in logs
* malformed events

## Concurrency tests

Test:

* concurrent events
* reconnect races
* duplicate events
* ordering issues
* cancellation
* shutdown behavior

---

# 20. EXTERNAL RESOURCES MUST BE MOCKABLE

No normal unit/integration test should require real Microsoft infrastructure.

Do NOT use real:

* Microsoft Graph
* Microsoft login
* external network services
* production accounts

during automated tests.

Create abstractions such as:

```rust
trait TeamsProvider {
    ...
}
```

and test implementations such as:

```text
MockTeamsProvider
FakeTeamsProvider
FixtureTeamsProvider
```

Use deterministic fixtures.

Tests must work:

* offline
* in CI
* without Microsoft credentials
* without network access
* deterministically

External integration tests, if ever added, must be explicitly separated from normal CI and clearly identified.

---

# 21. MOCK SERVER / HTTP TESTING

Where appropriate, introduce a mock HTTP layer for testing the Microsoft Graph adapter.

Tests should be able to simulate:

* HTTP 200
* HTTP 201
* HTTP 204
* HTTP 400
* HTTP 401
* HTTP 403
* HTTP 404
* HTTP 409
* HTTP 429
* HTTP 500
* HTTP 502
* HTTP 503
* timeouts
* malformed JSON
* incomplete responses
* pagination internally
* subscription expiration
* duplicate events
* out-of-order events

Never couple the domain layer to HTTP.

---

# 22. PERFORMANCE

Performance matters.

The TUI must remain responsive.

Avoid:

* blocking operations on the UI thread
* unnecessary allocations
* excessive cloning
* repeated parsing
* unnecessary API calls
* uncontrolled event queues
* unbounded memory growth
* excessive redraws

Evaluate:

* message rendering cost
* large conversations
* large messages
* rapid incoming event streams
* reconnect storms
* cache size
* search performance
* Unicode handling
* terminal resize events

Use profiling/benchmarks where appropriate.

Do not prematurely optimize.

Measure before making complicated optimizations.

---

# 23. ASYNCHRONOUS ARCHITECTURE

Use an appropriate Rust async runtime and justify the decision.

Network and background operations must not block the TUI.

Define clear boundaries between:

* UI event loop
* background tasks
* network requests
* event subscriptions
* synchronization
* application state updates

Handle:

* cancellation
* graceful shutdown
* task failure
* task supervision
* reconnects
* backpressure

Avoid spawning uncontrolled background tasks.

---

# 24. ERROR HANDLING

Define a coherent error architecture.

Errors should be:

* typed where appropriate
* contextual
* actionable
* safe to display
* safe to log
* free of secrets

Separate:

```text
Domain Errors
Infrastructure Errors
Authentication Errors
Provider Errors
Network Errors
UI Errors
Configuration Errors
Security Errors
```

where justified.

Never expose raw sensitive API responses to users.

---

# 25. LOGGING

Implement production-quality diagnostics.

Support:

* structured logging
* configurable log level
* `RUST_LOG` or equivalent
* `--verbose`
* `--debug`
* safe log files
* privacy-aware diagnostics

Never log:

* access tokens
* refresh tokens
* passwords
* secrets
* authentication codes
* authorization headers
* private credentials

Consider redaction at the logging boundary.

---

# 26. CLI

Provide a clean CLI foundation.

At minimum evaluate commands such as:

```bash
rusteams
rusteams login
rusteams logout
rusteams status
rusteams doctor
rusteams config
rusteams version
```

Only implement commands that make sense.

Do not create command bloat.

The interactive TUI is the primary user experience.

---

# 27. CONFIGURATION

Use the following configuration precedence unless analysis proves a better approach:

```text
CLI arguments
      ↓
Environment variables
      ↓
Configuration file
      ↓
Safe defaults
```

Prefer TOML for user configuration unless a strong technical reason suggests otherwise.

Never store secrets in normal configuration files.

Clearly document:

* configuration location
* supported variables
* precedence
* defaults
* security considerations

---

# 28. FILESYSTEM SECURITY

If local files are created:

* use safe paths
* prevent path traversal
* avoid unsafe symlink behavior
* use restrictive permissions when appropriate
* avoid predictable temporary files
* never overwrite arbitrary files
* validate configuration locations

Do not assume a local filesystem is trustworthy.

---

# 29. CROSS-PLATFORM SUPPORT

Mandatory:

## Linux

* x86_64
* ARM64 / aarch64

Optional, only if justified by implementation effort and dependency support:

## macOS

* x86_64
* ARM64

## Windows

* x86_64
* ARM64

Do not allow optional platform support to compromise the mandatory Linux implementation.

The architecture should avoid unnecessary platform-specific code.

Where platform-specific behavior is necessary, isolate it behind explicit abstractions.

---

# 30. RUST TOOLCHAIN

Establish an explicit MSRV policy.

Create and maintain:

```text
rust-toolchain.toml
```

where appropriate.

Use a stable Rust release unless a compelling reason exists otherwise.

Use the appropriate Rust edition.

Document:

* MSRV
* supported toolchain
* build requirements
* platform dependencies

CI must validate the MSRV policy.

---

# 31. GPLv3

The entire project must be released under **GNU GPL v3**.

Include:

```text
LICENSE
COPYING
```

and appropriate SPDX/license metadata.

Audit dependencies for license compatibility.

Do not introduce dependencies whose licenses conflict with GPLv3 without explicitly identifying the issue and stopping for a decision.

---

# 32. REPOSITORY STRUCTURE

Establish a professional repository layout.

A possible starting point:

```text
rusteams/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── security.yml
│   │   ├── release.yml
│   │   └── ...
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── dependabot.yml
│
├── docs/
│   ├── architecture/
│   ├── security/
│   ├── development/
│   ├── api/
│   ├── phases-0.md
│   ├── phases-1.md
│   ├── phases-2.md
│   ├── phases-3.md
│   └── ...
│
├── src/
├── tests/
├── benches/
├── examples/
│
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── rustfmt.toml
├── clippy.toml
├── deny.toml
├── LICENSE
├── COPYING
├── README.md
├── SECURITY.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

This structure is not mandatory if your architectural analysis identifies a superior organization.

---

# 33. PROVIDER ARCHITECTURE

Create a provider abstraction suitable for future growth.

The architecture should conceptually allow capabilities such as:

```text
Provider
├── Authentication
├── Users
├── Presence
├── Chats
├── Messages
├── Teams
├── Channels
├── Meetings
├── Calls
├── Files
├── Calendar
├── Notifications
└── Search
```

Do NOT implement all of these now.

The MVP should implement only what is required.

Avoid speculative abstractions with no demonstrated value.

The provider abstraction exists to prevent Microsoft Graph details from contaminating the application/domain/TUI layers.

---

# 34. FUTURE 1.0.0 SCOPE

The 1.0.0 target is NOT defined as literal GUI feature parity with Microsoft Teams.

Instead:

> Implement functionality that is realistically exposed through supported Microsoft APIs, especially Microsoft Graph, and that can meaningfully be represented through a terminal interface.

Recognize terminal limitations.

Some Teams capabilities may not make sense in a TUI.

For example, features requiring rich visual media may need:

* text-based alternatives
* external browser handoff
* terminal hyperlinks
* explicit unsupported-state messaging
* integration with external applications

Do not pretend terminal software can reproduce graphical Teams functionality perfectly.

The roadmap should distinguish:

```text
Fully TUI-compatible
Partially TUI-compatible
Browser-assisted
External application-assisted
Not meaningfully compatible
Unsupported by available APIs
```

This classification must be maintained as the project evolves.

---

# 35. LONG-TERM FEATURES

Investigate and document future possibilities including, where realistically supported:

* teams
* channels
* channel messages
* chat
* group chat
* presence
* notifications
* search
* files
* OneDrive/SharePoint integration
* meetings
* calendar
* calls
* meeting participation
* audio-related workflows
* video-related workflows
* meeting metadata
* recordings
* transcripts
* captions
* reactions
* mentions
* bots/apps
* workflows
* permissions
* administrative functionality

Do not promise unsupported functionality.

Each future capability must be classified by:

1. Microsoft API availability
2. Required permissions
3. Authentication requirements
4. TUI feasibility
5. Security implications
6. Performance implications
7. Implementation complexity
8. External application dependencies

---

# 36. PHASED ROADMAP

Create detailed phase documentation under:

```text
docs/phases-X.md
```

The roadmap must be comprehensive and long-term.

However, **the initial phases must receive substantially more detail than later phases**.

As the project matures, later phase documents should be expanded and refined based on discoveries made during implementation.

Do not write superficial phase documents.

---

# 37. REQUIRED PHASE DOCUMENT STRUCTURE

Every phase document should contain, where applicable:

```markdown
# Phase X — <Name>

## Objective

## Motivation

## Scope

## Non-Goals

## Dependencies

## User Stories

## Functional Requirements

## Non-Functional Requirements

## Architecture

## Components

## Interfaces

## Data Models

## State Machines

## API Integration

## Authentication

## Security Requirements

## Privacy Requirements

## Performance Requirements

## Error Handling

## Concurrency

## Persistence

## TUI/UX

## Accessibility

## Testing Strategy

## TDD Scenarios

## Integration Tests

## Security Tests

## Acceptance Criteria

## Risks

## Trade-offs

## Open Questions

## Migration Considerations

## Developer Notes

## Definition of Done

## Future Evolution
```

Adapt the sections where they do not apply.

---

# 38. INITIAL PHASES

The first phases should be extremely detailed.

At minimum define phases for:

## Phase 0 — Discovery, Architecture, Threat Model and Foundation

Cover:

* requirements
* constraints
* API research
* authentication research
* architecture alternatives
* architecture decision records
* threat model
* repository foundation
* Rust toolchain
* CI
* dependency policy
* licensing
* testing infrastructure
* TDD infrastructure
* mock provider
* project conventions

## Phase 1 — Core Domain and Application Architecture

Define:

* domain models
* application state
* commands
* events
* provider traits
* error model
* synchronization model
* concurrency architecture
* mock infrastructure
* core TDD scenarios

## Phase 2 — Authentication

Implement:

* Microsoft authentication
* secure token lifecycle
* account state
* authentication UX
* authentication errors
* security testing

## Phase 3 — Chat Read Experience

Implement:

* conversation discovery
* conversation list
* message history
* message rendering
* presence
* search
* initial TUI

## Phase 4 — Sending and Message Operations

Implement:

* sending
* editing
* deletion
* reply
* reactions
* mentions
* rich text
* emojis
* files/attachments

## Phase 5 — Real-Time Event Architecture

Implement:

* event subscriptions
* event processing
* state updates
* reconnect
* synchronization
* subscription renewal
* duplicate/out-of-order handling

## Phase 6 — Notifications and UX Hardening

Implement:

* notifications
* status handling
* command palette
* keyboard UX
* accessibility
* resilience
* large conversation performance

## Phase 7+ — Future Teams Capabilities

Define strategic specifications initially.

Do not over-engineer implementation before earlier phases reveal the actual requirements.

Later phases must become increasingly detailed as development progresses.

---

# 39. ARCHITECTURE DECISION RECORDS

Create ADRs for significant architectural decisions.

Examples:

```text
docs/architecture/
├── adr-001-architecture.md
├── adr-002-provider-abstraction.md
├── adr-003-authentication.md
├── adr-004-async-runtime.md
├── adr-005-state-management.md
├── adr-006-realtime-events.md
├── adr-007-cache-strategy.md
├── adr-008-terminal-security.md
└── ...
```

Do not create ADRs mechanically for trivial decisions.

Each ADR should explain:

* context
* decision
* alternatives
* rationale
* consequences
* security implications
* future implications

---

# 40. ENGINEERING QUALITY GATES

Every meaningful phase must pass:

## Gate 1 — Requirements

Requirements are understood and documented.

## Gate 2 — Architecture

Architecture is coherent and justified.

## Gate 3 — Security

Threats and mitigations are documented.

## Gate 4 — TDD

Tests exist before implementation for new behavior.

## Gate 5 — Implementation

Feature works according to acceptance criteria.

## Gate 6 — Testing

Unit/integration/security tests pass.

## Gate 7 — Quality

Run appropriate:

```bash
cargo fmt
cargo check
cargo clippy
cargo test
```

plus relevant security, coverage, benchmark, and integration tooling.

## Gate 8 — Documentation

Documentation accurately reflects implementation.

## Gate 9 — Regression

Existing functionality remains correct.

---

# 41. CI/CD

Create professional CI.

At minimum evaluate:

* formatting
* compilation
* unit tests
* integration tests
* clippy
* MSRV
* Linux x86_64
* Linux ARM64
* dependency auditing
* license auditing
* security checks
* documentation checks
* release builds

Optional platforms should be evaluated without making them mandatory unless practical.

---

# 42. SUPPLY-CHAIN SECURITY

Treat dependencies as part of the attack surface.

Evaluate:

* `cargo audit`
* `cargo deny`
* lockfile policy
* dependency freshness
* license compatibility
* known vulnerabilities
* unmaintained dependencies
* dependency count
* unnecessary dependencies

Prefer a small and well-maintained dependency graph.

Do not add a dependency for trivial functionality that can be implemented safely and clearly in a few lines.

Conversely, do not reinvent complex cryptography or security primitives.

Use mature, audited libraries for security-critical functionality.

---

# 43. API PERMISSION MINIMIZATION

Use the minimum Microsoft Graph permissions required for each capability.

Document:

* permission
* purpose
* security impact
* whether delegated/application permission is required
* why it is necessary

Do not request broad permissions merely for future convenience.

When future features require additional permissions, document them separately.

---

# 44. PRIVACY

Treat Teams data as sensitive enterprise information.

Follow data minimization.

Avoid collecting telemetry unless explicitly specified later.

Do not introduce analytics by default.

Do not transmit user data to third parties.

Do not persist messages unnecessarily.

Document what `rusteams` stores locally.

---

# 45. TERMINAL DATA SANITIZATION

Create a dedicated security boundary for rendering untrusted text.

Conceptually:

```text
Remote Teams Data
       │
       ▼
Untrusted Input
       │
       ▼
Validation / Sanitization
       │
       ▼
Safe Presentation Model
       │
       ▼
Ratatui Renderer
       │
       ▼
Terminal
```

Never render raw remote strings directly into terminal escape-capable output without considering control sequences.

Create comprehensive tests for this.

This is a mandatory security feature.

---

# 46. MESSAGE REPRESENTATION

Do not assume Markdown and Microsoft Teams rich text are identical.

Research Microsoft's actual message content representation.

Define a normalized internal representation.

The renderer should translate remote content into safe terminal presentation.

Handle:

* plain text
* links
* mentions
* formatting
* code
* lists
* emojis
* attachments
* files

Do not claim unsupported formatting fidelity.

---

# 47. SEARCH

Search must be designed independently from the TUI.

Define:

```text
SearchQuery
SearchFilters
SearchResult
SearchService
```

where appropriate.

Do not implement search by downloading an unbounded amount of data and searching locally unless explicitly justified.

Respect API capabilities and throttling.

---

# 48. MESSAGE HISTORY

Message history must remain responsive.

Investigate the API's pagination model internally.

Even though user-visible pagination is not an MVP requirement, internal pagination may be mandatory.

Implement lazy loading where beneficial.

Avoid loading unbounded message history into memory.

---

# 49. RATE LIMITING

Microsoft APIs may impose throttling.

Design around:

* HTTP 429
* retry-after
* exponential backoff
* request coalescing
* duplicate requests
* unnecessary refreshes

Never hammer Microsoft Graph after a rate-limit response.

Tests must simulate throttling.

---

# 50. SHUTDOWN

Implement graceful shutdown.

On exit:

* stop background tasks
* close event streams
* stop reconnect loops
* flush appropriate logs
* securely clean temporary sensitive state
* restore terminal state
* leave the terminal usable

The application must not leave the user's terminal in raw/alternate mode after crashes where practical.

---

# 51. TERMINAL FAILURE RESILIENCE

Consider:

* terminal resize
* terminal disconnect
* SIGINT
* SIGTERM where applicable
* panic handling
* rendering errors
* unsupported terminal capabilities
* narrow terminals
* Unicode limitations

Provide safe recovery.

---

# 52. DOCUMENTATION

Create excellent documentation.

At minimum:

```text
README.md
CONTRIBUTING.md
SECURITY.md
CHANGELOG.md
docs/
```

README should explain:

* what rusteams is
* why it exists
* status
* MVP scope
* architecture overview
* installation
* authentication
* usage
* configuration
* keyboard shortcuts
* security
* supported platforms
* limitations
* roadmap
* contributing
* license

Do not advertise functionality that does not exist.

---

# 53. DEVELOPER DOCUMENTATION

Document:

* architecture
* development setup
* testing
* TDD workflow
* mocking
* provider implementation
* adding features
* error handling
* security model
* release process
* dependency policy

The project should be understandable to developers who did not build it.

---

# 54. CHANGE MANAGEMENT

Maintain:

```text
CHANGELOG.md
```

Use a consistent versioning strategy.

Follow semantic versioning where appropriate.

Clearly distinguish:

* breaking changes
* features
* fixes
* security changes
* internal changes

---

# 55. VERSIONING

Begin with an appropriate pre-1.0 version.

Do not claim `1.0.0` until the documented 1.0 criteria are satisfied.

MVP should be explicitly identified as an MVP.

The roadmap should make clear that:

```text
MVP ≠ 1.0.0
```

---

# 56. NO PREMATURE FEATURE EXPANSION

Do not implement future Teams features simply because they are interesting.

Prioritize:

```text
Architecture
↓
Security
↓
Authentication
↓
Core chat
↓
Realtime
↓
Reliability
↓
UX
↓
Performance
↓
Future features
```

The project must become excellent at chat before expanding horizontally.

---

# 57. DEFINITION OF DONE

A feature is NOT complete merely because it compiles.

A feature is complete only when:

* requirements are documented
* architecture is appropriate
* implementation works
* unit tests exist
* integration tests exist where appropriate
* security considerations are addressed
* failure modes are tested
* performance is acceptable
* documentation is updated
* CI passes
* formatting passes
* clippy passes
* no known secrets are exposed
* no unjustified technical debt is introduced

---

# 58. DEVELOPMENT LOOP

For every feature:

```text
1. Understand requirement
2. Check authoritative API documentation
3. Identify ambiguities
4. Ask user only if genuinely necessary
5. Update phase documentation
6. Define acceptance criteria
7. Define threat model implications
8. Design tests
9. Write failing tests
10. Implement minimum functionality
11. Make tests pass
12. Refactor
13. Add integration tests
14. Add security tests
15. Evaluate performance
16. Update documentation
17. Run quality gates
18. Review architecture
19. Update ADRs if necessary
20. Continue autonomously
```

---

# 59. IMPORTANT: DO NOT CREATE FAKE FUNCTIONALITY

Never implement fake Teams functionality simply to make the application look complete.

Do not:

* fabricate messages
* fabricate users
* fabricate realtime events
* fabricate API responses outside tests
* pretend authentication succeeded
* create fake production behavior

Mocks/fakes are acceptable ONLY in tests or explicitly documented development/demo environments.

Production behavior must represent real supported functionality.

---

# 60. AUTHORITATIVE INFORMATION

When investigating Microsoft APIs, prioritize authoritative sources.

Prefer:

1. Microsoft Learn
2. Microsoft Graph documentation
3. Microsoft Entra documentation
4. Official Rust documentation
5. Official crate documentation/repositories
6. Relevant RFCs/specifications

Do not base critical architectural decisions on random blog posts when authoritative documentation exists.

---

# 61. API FEASIBILITY AUDIT

Before implementing each Microsoft Teams feature, determine:

```text
Feature
│
├── Is it exposed by Microsoft Graph?
│
├── Is the API officially supported?
│
├── What permissions are required?
│
├── Is it available to delegated users?
│
├── What account/tenant limitations exist?
│
├── Does realtime support exist?
│
├── What are throttling constraints?
│
├── What data representation is returned?
│
├── Can the feature reasonably work in a TUI?
│
└── What security/privacy implications exist?
```

Document the answer.

If the feature is impossible or unsupported, document it rather than inventing an implementation.

---

# 62. FINAL ROADMAP PHILOSOPHY

The project roadmap must explicitly recognize:

```text
Microsoft Teams
       │
       ▼
Microsoft Graph capabilities
       │
       ▼
rusteams provider capabilities
       │
       ▼
Domain capabilities
       │
       ▼
Terminal-compatible capabilities
       │
       ▼
Actual implemented features
```

These layers are not identical.

The roadmap must never equate:

> “Teams supports X”

with:

> “rusteams can implement X.”

---

# 63. START NOW

Your first autonomous actions should be:

## Step 1

Inspect the repository/environment.

## Step 2

Establish the project requirements and constraints.

## Step 3

Research the current official Microsoft Graph capabilities relevant to the MVP.

Especially investigate:

* authentication
* chats
* messages
* message editing
* message deletion
* replies
* reactions
* mentions
* files
* attachments
* search
* presence
* read receipts
* notifications
* realtime/change notifications
* subscriptions
* synchronization
* throttling

## Step 4

Identify API limitations.

## Step 5

Create the architecture proposal.

## Step 6

Create the threat model.

## Step 7

Create initial ADRs.

## Step 8

Create the repository foundation.

## Step 9

Create:

```text
docs/phases-0.md
docs/phases-1.md
docs/phases-2.md
...
```

Initial phases must be highly detailed.

Later phases should be strategically detailed but explicitly marked for refinement.

## Step 10

Implement Phase 0 using TDD where applicable.

## Step 11

Validate the foundation.

## Step 12

Proceed autonomously through subsequent phases.

---

# 64. WHEN YOU ENCOUNTER AN AMBIGUITY

Use this exact decision hierarchy:

```text
Can existing requirements answer it?
        │
       YES ──► proceed
        │
       NO
        ▼
Can authoritative documentation answer it?
        │
       YES ──► proceed and document
        │
       NO
        ▼
Can standard engineering practice safely resolve it?
        │
       YES ──► make the decision and document it
        │
       NO
        ▼
STOP AND ASK THE USER
```

Do not ask questions unnecessarily.

Do not make dangerous assumptions.

---

# 65. QUALITY STANDARD

Do not optimize for:

> “make something that works.”

Optimize for:

> **“build a secure, maintainable, testable, performant, extensible, enterprise-grade Rust application that can realistically evolve into the long-term rusteams vision.”**

Act as though this project will be maintained for many years by developers who were not involved in its initial implementation.

Every shortcut must be justified.

Every security-sensitive decision must be explicit.

Every architectural decision must have a reason.

Every important behavior must have tests.

Every externally observable feature must have documentation.

Every limitation must be honestly documented.

---

# 66. FINAL INSTRUCTION

Begin by analyzing the repository and requirements.

Do not immediately generate a large amount of application code.

First establish:

1. Requirements
2. API feasibility
3. Architecture
4. Threat model
5. Testing strategy
6. Repository structure
7. Phase roadmap
8. Initial implementation plan

Then begin implementation.

Work autonomously.

Use strict TDD.

Use mocks for all external resources during automated tests.

Prefer secure defaults.

Prefer minimal permissions.

Prefer minimal dependencies.

Prefer explicit boundaries.

Prefer boring, reliable engineering over cleverness.

Do not fabricate functionality.

Do not hide limitations.

Do not silently guess.

Build `rusteams` as a serious open-source project.

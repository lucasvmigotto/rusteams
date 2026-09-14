# ADR-007 — Cache strategy (ephemeral by default)

- Status: accepted
- Context: Teams messages are sensitive enterprise data.
- Decision: in-memory state only in MVP; no persistent message cache until a
  retention/invalidation/minimization ADR justifies it. `Clear` path required
  if persistence ever lands.

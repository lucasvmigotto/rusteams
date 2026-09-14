# Incident response

1. Triage: confirm scope (token leak vs rendering injection vs dependency CVE).
2. Contain: revoke affected refresh tokens (`logout`), rotate client IDs if shared.
3. Fix: TDD regression test first, then patch, then `cargo audit` re-run.
4. Disclose: SECURITY.md contact, 72h ack target, credit reporter.
5. Learn: update threat model + ADR.

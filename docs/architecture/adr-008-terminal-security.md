# ADR-008 — Terminal security boundary

- Status: accepted
- Context: malicious messages must never drive terminal control sequences.
- Decision: `sanitize()` is the sole path from remote text to rendering —
  strips C0 (keeps \n,\t), DEL, CSI/OSC/Fe sequences, non-characters.
  ratatui receives only sanitized output. Covered by unit tests; proptest next.

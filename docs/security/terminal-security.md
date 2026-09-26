# Terminal security

All remote strings pass `sanitize()` before ratatui. Stripped: C0 controls
(except \n, \t), DEL, CSI (`ESC [`), OSC (`ESC ]`…BEL/`ESC \`), Fe escapes,
charset designators, Unicode non-characters. See `src/sanitize.rs` tests.
Never render raw Graph `body.content`; never enable terminal hyperlinks from
remote data without an allowlist (none in MVP).

# Gate 3 No-Leak and Accessibility Checklist

Review date: 2026-06-06

Scope: `apps/web` served `dist` shell for `race_to_n` / Race to 21, `three_marks` / Three Marks, and `column_four` / Column Four.

## Accessibility Baseline

- Keyboard path: game setup, start match, legal action, developer panel, stale diagnostic, replay export/import, and replay step are reachable through keyboard focus.
- Focus visibility: buttons, inputs, selects, textareas, and custom mode radio labels expose a visible focus outline.
- Accessible names: interactive controls have text, label text, or `aria-label`.
- Reduced motion: the smoke emulates `prefers-reduced-motion: reduce`; effect entries and counter transitions preserve information without relying on animation.
- Non-color cues: turn state, diagnostics, and effects include text labels in addition to tone/color styling.
- Responsive smoke: the no-leak/a11y smoke runs the critical path at a narrow mobile-sized viewport.
- Three Marks board keyboard path: the Three Marks smoke focuses a legal cell and activates it with Enter; occupied cells are asserted inert.
- Three Marks non-color cues: marks use distinct shape plus color, terminal state has text, and winning cells are highlighted with text status.
- Three Marks replay accessibility: replay reset/step renders a board projection and command sequence rather than JSON-only state.
- Column Four board keyboard path: the Column Four smoke focuses a Rust-legal column control, exposes the Rust landing preview, and activates it with Enter.
- Column Four legal-control accessibility: the smoke asserts exactly seven named column controls, full columns are inert, and terminal boards expose no playable controls.
- Column Four non-color cues: seat pieces use distinct shape plus color, terminal winner/draw text is visible, and the Rust winning line is highlighted with terminal status text.
- Column Four reduced motion: the smoke verifies reduced-motion mode suppresses landed-piece animation while preserving the Rust-projected board state.
- Column Four replay accessibility: replay reset/step renders `ColumnFourBoard` and the public command sequence rather than JSON-only state.

## No-Leak Surfaces

- DOM text and attributes: checked by `a11y-noleak.smoke.mjs` for hidden/private/internal leak vocabulary.
- `data-testid` attributes: checked by `a11y-noleak.smoke.mjs`; test IDs remain generic UI hooks, not state dumps.
- Console logs and page errors: captured by `a11y-noleak.smoke.mjs` and checked for forbidden leak vocabulary.
- `localStorage` / `sessionStorage`: session storage must remain empty; local storage may contain only the `rulepath.reducedMotion` UI preference with `reduce` or `motion`.
- Developer panel: limited to API version, operation count, selected public game, match id, seed, mode, active actor, freshness token, action count, effect cursor/count, pending op, replay id/cursor, and public diagnostics.
- Replay export: checked for forbidden leak vocabulary. The perfect-information replay document may include `expected_private_view_hashes.not_applicable` as an explicit schema marker; it does not contain private view payloads.
- Three Marks DOM/test IDs: checked by `three-marks.smoke.mjs` for hidden/private/internal leak vocabulary; board test IDs identify public cells only (`r1c1` through `r3c3`) and never carry state dumps.
- Three Marks bot explanation: the UI shows only the Rust-provided public `bot_chose_action` explanation; no candidate ranking or hidden search surface is exposed.
- Column Four DOM/test IDs: checked by `column-four.smoke.mjs` for hidden/private/internal leak vocabulary; board test IDs identify public column controls only (`c1` through `c7`) and never carry state dumps.
- Column Four replay export: checked by `column-four.smoke.mjs` for forbidden leak vocabulary while preserving public replay metadata for `column_four`.
- Column Four bot rationale: the UI shows only the Rust-provided public `bot_chose_action` rationale; no candidate ranking, raw score, hidden search, or internal state surface is exposed.

## Later Hidden-Information Games

- Keep browser surfaces whitelist-oriented: public view, public action labels, public effects, public diagnostics, replay command/checkpoint metadata, and explicit not-applicable markers.
- Do not add hidden hand/state data, private bot reasoning, candidate rankings, internal snapshots, or full engine state to DOM, attributes, storage, console output, replay textareas, or developer panels.

# Gate 3 No-Leak and Accessibility Checklist

Review date: 2026-06-06

Scope: `apps/web` served `dist` shell for `race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column Four, and `directional_flip` / Directional Flip.

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
- Directional Flip board keyboard path: the Directional Flip smokes focus the grid, move with arrow keys to a Rust-legal target, activate with Enter, and assert the Rust-projected ply advances.
- Directional Flip legal-cell accessibility: the smoke asserts 64 named grid cells, four Rust legal targets in the opening position, forced-pass replay projection, and accessible text status.
- Directional Flip non-color cues: discs use distinct SVG marks/patterns plus color, score/status/effect text is visible, and legal/preview cells are backed by Rust labels.
- Directional Flip reduced motion: the smoke verifies reduced-motion mode suppresses flip animation while preserving the Rust-projected board state.
- Directional Flip replay accessibility: replay reset/step renders `DirectionalFlipBoard`, forced-pass projection, and the public command sequence rather than JSON-only state.

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
- Directional Flip DOM/test IDs: checked by `directional-flip.smoke.mjs` for hidden/private/internal leak vocabulary; board test IDs identify public cells only (`r1c1` through `r8c8`) and never carry state dumps.
- Directional Flip replay export: checked by `directional-flip.smoke.mjs` for forbidden leak vocabulary while preserving public replay metadata for `directional_flip` and explicit not-applicable private-view markers.
- Directional Flip bot rationale: the UI shows only the Rust-provided public `bot_chose_action` rationale; no candidate ranking, raw score, hidden search, or internal state surface is exposed.
- Directional Flip forced-pass replay: the smoke imports the Rust golden forced-pass trace and verifies the browser projects the forced-pass state/control without exposing private or internal state.

## Later Hidden-Information Games

- Keep browser surfaces whitelist-oriented: public view, public action labels, public effects, public diagnostics, replay command/checkpoint metadata, and explicit not-applicable markers.
- Do not add hidden hand/state data, private bot reasoning, candidate rankings, internal snapshots, or full engine state to DOM, attributes, storage, console output, replay textareas, or developer panels.

# Gate 3 No-Leak and Accessibility Checklist

Review date: 2026-06-06

Scope: `apps/web` served `dist` shell for `race_to_n` / Race to 21 and `three_marks` / Three Marks.

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

## No-Leak Surfaces

- DOM text and attributes: checked by `a11y-noleak.smoke.mjs` for hidden/private/internal leak vocabulary.
- `data-testid` attributes: checked by `a11y-noleak.smoke.mjs`; test IDs remain generic UI hooks, not state dumps.
- Console logs and page errors: captured by `a11y-noleak.smoke.mjs` and checked for forbidden leak vocabulary.
- `localStorage` / `sessionStorage`: session storage must remain empty; local storage may contain only the `rulepath.reducedMotion` UI preference with `reduce` or `motion`.
- Developer panel: limited to API version, operation count, selected public game, match id, seed, mode, active actor, freshness token, action count, effect cursor/count, pending op, replay id/cursor, and public diagnostics.
- Replay export: checked for forbidden leak vocabulary. The perfect-information replay document may include `expected_private_view_hashes.not_applicable` as an explicit schema marker; it does not contain private view payloads.
- Three Marks DOM/test IDs: checked by `three-marks.smoke.mjs` for hidden/private/internal leak vocabulary; board test IDs identify public cells only (`r1c1` through `r3c3`) and never carry state dumps.
- Three Marks bot explanation: the UI shows only the Rust-provided public `bot_chose_action` explanation; no candidate ranking or hidden search surface is exposed.

## Later Hidden-Information Games

- Keep browser surfaces whitelist-oriented: public view, public action labels, public effects, public diagnostics, replay command/checkpoint metadata, and explicit not-applicable markers.
- Do not add hidden hand/state data, private bot reasoning, candidate rankings, internal snapshots, or full engine state to DOM, attributes, storage, console output, replay textareas, or developer panels.

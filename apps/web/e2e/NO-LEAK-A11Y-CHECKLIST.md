# Gate 3 No-Leak and Accessibility Checklist

Review date: 2026-06-06

Scope: `apps/web` served `dist` shell for `race_to_n` / Race to 21.

## Accessibility Baseline

- Keyboard path: game setup, start match, legal action, developer panel, stale diagnostic, replay export/import, and replay step are reachable through keyboard focus.
- Focus visibility: buttons, inputs, selects, textareas, and custom mode radio labels expose a visible focus outline.
- Accessible names: interactive controls have text, label text, or `aria-label`.
- Reduced motion: the smoke emulates `prefers-reduced-motion: reduce`; effect entries and counter transitions preserve information without relying on animation.
- Non-color cues: turn state, diagnostics, and effects include text labels in addition to tone/color styling.
- Responsive smoke: the no-leak/a11y smoke runs the critical path at a narrow mobile-sized viewport.

## No-Leak Surfaces

- DOM text and attributes: checked by `a11y-noleak.smoke.mjs` for hidden/private/internal leak vocabulary.
- `data-testid` attributes: checked by `a11y-noleak.smoke.mjs`; test IDs remain generic UI hooks, not state dumps.
- Console logs and page errors: captured by `a11y-noleak.smoke.mjs` and checked for forbidden leak vocabulary.
- `localStorage` / `sessionStorage`: session storage must remain empty; local storage may contain only the `rulepath.reducedMotion` UI preference with `reduce` or `motion`.
- Developer panel: limited to API version, operation count, selected public game, match id, seed, mode, active actor, freshness token, action count, effect cursor/count, pending op, replay id/cursor, and public diagnostics.
- Replay export: checked for forbidden leak vocabulary. The perfect-information replay document may include `expected_private_view_hashes.not_applicable` as an explicit schema marker; it does not contain private view payloads.

## Later Hidden-Information Games

- Keep browser surfaces whitelist-oriented: public view, public action labels, public effects, public diagnostics, replay command/checkpoint metadata, and explicit not-applicable markers.
- Do not add hidden hand/state data, private bot reasoning, candidate rankings, internal snapshots, or full engine state to DOM, attributes, storage, console output, replay textareas, or developer panels.

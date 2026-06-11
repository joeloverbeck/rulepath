# Rulepath No-Leak and Accessibility Checklist

Review date: 2026-06-07

Scope: `apps/web` served `dist` shell for `race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column Four, `directional_flip` / Directional Flip, and `draughts_lite` / Draughts Lite.

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
- Draughts Lite board keyboard path: the Draughts Lite smoke focuses the grid, moves with arrows to a Rust-legal origin, activates with Enter, moves to a Rust-provided destination, and activates with Space.
- Draughts Lite compound path accessibility: pending origin, destination count, mandatory capture, capture destination count, and replay command paths are exposed as text with full multi-segment paths.
- Draughts Lite legal-cell accessibility: the smoke asserts 64 named grid cells, four Rust legal origins in the opening position, active-descendant state, visible focus, and accessible piece/cell labels.
- Draughts Lite non-color cues: pieces use shape/text marks plus color, selected origins/legal destinations/captures/promotions have text status and live-region cues, and effects include text labels.
- Draughts Lite reduced motion: the smoke verifies reduced-motion mode suppresses Draughts cell transitions while preserving static highlights and the effect log.
- Draughts Lite replay accessibility: replay export/import/step renders `DraughtsLiteBoard` and full public command paths such as `from/r4c1 > jump/r6c3`.
- Veiled Draft board keyboard path: the Secret Draft smoke focuses a Rust-legal visible-pool commit control, activates it with Enter, repeats for the second seat, and verifies the grouped reveal.
- Veiled Draft pending/reveal accessibility: pending seat statuses, priority, score, visible pool, drafted collections, and reveal history are exposed as text with non-color cues.
- Veiled Draft reduced motion: the smoke verifies reduced-motion mode suppresses item/reveal animation while preserving pending/reveal order.
- Veiled Draft replay accessibility: replay import/reset/step renders the public observer timeline with redacted command summaries rather than raw command paths.
- Flood Watch cooperative path: the Flood Watch smoke covers forecast reveal, role labels, multi-action budget spending, environment-phase effects, human-vs-bot teammate automation, bot-vs-bot stepping, shared win/loss terminals, replay import/export, reduced motion, and responsive layout.

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
- Draughts Lite DOM/test IDs: checked by `draughts-lite.smoke.mjs` for hidden/private/internal leak vocabulary; board test IDs identify public cells only (`r1c1` through `r8c8`) and never carry state dumps.
- Draughts Lite replay export: checked by `draughts-lite.smoke.mjs` for forbidden leak vocabulary while preserving public replay metadata for `draughts_lite` and explicit not-applicable private-view markers.
- Draughts Lite bot rationale/effects: the UI shows only Rust-provided public effects and bot rationale; no candidate ranking, raw score, hidden search, or internal state surface is exposed.
- Draughts Lite forced capture: the smoke creates a standard-match mandatory capture through public UI moves and verifies the DOM/live text exposes only Rust public legality cues and complete replay segments.
- Veiled Draft pre-reveal DOM/test IDs: checked by `secret-draft.smoke.mjs`; committed item ids such as `ember_1` and `commit/ember_1` are absent from DOM text, attributes, `data-testid` values, storage, and console before reveal.
- Veiled Draft pending UI: post-commit anchors use seat/round identifiers, and the board shows only committed/waiting state until Rust emits the grouped reveal.
- Veiled Draft replay export/import: default export is `viewer_scoped_observation_v1`, omits command stream and seed evidence, and replay viewer consumes public effects/redacted command summaries.
- Veiled Draft bot/effect text: the Human vs bot smoke verifies Rust's automatic bot commitment reaches grouped reveal without candidate ranking, hidden state, private state, or internal debug text.
- Flood Watch DOM/test IDs/storage/logs: checked by `flood-watch.smoke.mjs`; district test IDs use public district IDs only, storage remains UI-only, and forbidden deck-order/internal-state terms are absent before and after terminal states.
- Flood Watch replay export/import: public export is observer scoped, includes redacted command summaries, omits raw commands and seed evidence, and never exposes full event-deck order.
- Flood Watch bot/effect text: browser smoke verifies the public cooperative bot policy id and Rust semantic storm effects without candidate rankings, private state, internal state, or deck-order payloads.

## Later Hidden-Information Games

- Keep browser surfaces whitelist-oriented: public view, public action labels, public effects, public diagnostics, replay command/checkpoint metadata, and explicit not-applicable markers.
- Do not add hidden hand/state data, private bot reasoning, candidate rankings, internal snapshots, or full engine state to DOM, attributes, storage, console output, replay textareas, or developer panels.

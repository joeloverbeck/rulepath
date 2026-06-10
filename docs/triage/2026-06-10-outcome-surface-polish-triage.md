# Outcome surface polish — triage (2026-06-10)

**Request**: the recently added OUTCOME section (VICEXPSHASUR series, 2026-06-09) looks notoriously poorer than the rest of the interface; explore games to terminal, diagnose, research usability/web-design practice, and create tickets.

**Method**: drove `http://127.0.0.1:4173/` with Puppeteer to terminal in Race to 21, Three Marks (win), and Crest Ledger (poker_lite, split); read the panel/adapter/templates code, the wasm-api rationale projections, UI-INTERACTION.md §16, and the archived `victory-explanation-shared-surface.md` spec; ran an external research pass (W3C/WCAG, ARIA APG, NN/g, GOV.UK, Polaris, Soueidan, web.dev, game post-match UI analyses).

**Classification**: dev-process (TS/CSS presentation + ticket authoring). The Rust rationale payload is deliberately untouched — changing it would move public-view/replay hashes.

## Findings

- **O1 — Root cause: the panel CSS was never authored.** `apps/web/src/styles.css` (2,884 lines) contains zero rules for the eight `outcome-*` classes emitted by `OutcomeExplanationPanel.tsx:100-183`. Everything renders with browser defaults: no card container, run-together standing headers ("seat_0Winner"), default `<dl>` indents, bulleted rule refs, unstyled disclosure buttons, no winner emphasis, no reduced-motion rules. The VICEXPSHASUR series delivered contract, data, component, wiring, and smokes — but no styling ticket existed. **Verdict: fix — OUTSURPOL-001.**
- **O2 — Raw internal tokens in player-facing copy.** `seat_0` in headings/summaries/standing labels (board call sites, e.g. `RaceBoard.tsx:58`, and the adapter fallback `OutcomeExplanationPanel.tsx:197`); raw enum tokens (`high_card`, `low`, `win`, `split`) via `formatValue`; raw cell ids (`r1c1`) in summaries; duplicated `Result:` rows under headers that already badge the result (duplication originates in the Rust projection; dedup at render time). **Verdict: fix — OUTSURPOL-002.**
- **O3 — Announcement/ordering/disclosure gaps.** The `role="status"` region mounts together with its content at terminal (unreliable announcement per WCAG 4.1.3 practice; boards already own a persistent `.board-status` live region to reuse); standings render in seat order, not winner-first; decisive/tiebreak breakdown sections are collapsed by default even when they decided the game. **Verdict: fix — OUTSURPOL-003.**
- **O4 — Palette constraint (design input, not a defect ticket).** Green `#2b8068` vs orange `#b45f06` luminance contrast is 1.04:1 — hue-only distinction; any green-vs-orange state distinction must co-occur with text/weight/border. Both pass AA as text on white (4.78:1 / 4.58:1); orange has no headroom for tints-as-text. **Verdict: encoded as invariants inside OUTSURPOL-001; no separate deliverable.**

## Deliverables

| Ticket | Scope | Priority |
|---|---|---|
| `archive/tickets/OUTSURPOL-001.md` | Author the missing outcome CSS (card, hierarchy, badges, winner emphasis, key-value grid, disclosure affordance, rule-ref chips, reduced motion, responsive) + styled-state smoke assertions | HIGH |
| `archive/tickets/OUTSURPOL-002.md` | Humanized copy: shared seat/enum display maps at the `formatValue`/`renderTemplate`/adapter chokepoints, 10-board call-site audit, render-time `Result:` dedup, template copy pass | HIGH |
| `tickets/OUTSURPOL-003.md` | Announcement via pre-existing live region, winner-first standings + draw parity, decisive-section `defaultOpen` | MEDIUM |

Dependency order: 001 and 002 are independent; 003 depends on both. All three are TS/CSS-only; an eventual Rust-side cleanup of the duplicated Result row is recorded as out of scope (hash migration cost).

## Key research anchors

WCAG 1.4.1 use-of-color & 1.4.11 non-text contrast; WCAG 4.1.3 status messages + ARIA22 (live region must pre-exist content); ARIA APG disclosure pattern (chevron, full-row button trigger); NN/g progressive disclosure ("disclose everything users frequently need up front"; ≤2 levels) and accordion guidance; GOV.UK details component (information-scent labels; don't hide majority-need content); Polaris badge tones (text-bearing status badges); key-value pair layout (≈40/60 split, group spacing > intra-pair spacing); post-match screen hierarchy (result headline ≫ standings ≫ justifying stats; winner-first ordering; emphasized winner card); chess-platform draw lessons ("Draw by stalemate" — always name the cause next to the result).

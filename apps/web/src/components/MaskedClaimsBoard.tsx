import { useMemo } from "react";
import type {
  ActionChoice,
  ActionTree,
  EffectEntry,
  MaskedClaimsMaskView,
  MaskedClaimsPublicView,
  SeatId,
} from "../wasm/client";
import { resolveSeatLabel } from "../seatLabels";
import { feedbackForEffect } from "./effectFeedback";
import { OutcomeExplanationPanel, outcomeAnnouncementText, outcomeSurfaceData } from "./OutcomeExplanationPanel";

type MaskedClaimsBoardProps = {
  view: MaskedClaimsPublicView;
  actionTree: ActionTree | null;
  latestEffect: EffectEntry | null;
  effects?: EffectEntry[];
  reducedMotion: boolean;
  pending: boolean;
  interactive?: boolean;
  onPathSubmit?: (path: string[]) => void;
};

export function MaskedClaimsBoard({
  view,
  actionTree,
  latestEffect,
  effects = latestEffect ? [latestEffect] : [],
  reducedMotion,
  pending,
  interactive = true,
  onPathSubmit,
}: MaskedClaimsBoardProps) {
  const claimChoices = useMemo(() => claimMaskChoices(actionTree), [actionTree]);
  const legalClaims = useMemo(() => new Map(claimChoices.map((choice) => [choice.segment, choice] as const)), [claimChoices]);
  const responseChoices = useMemo(() => responseActionChoices(actionTree), [actionTree]);
  const feedback = latestEffect ? feedbackForEffect(latestEffect) : null;
  const terminal = view.terminal.kind !== "non_terminal";
  const revealActive = effects.some((entry) => isRevealEffect(entry.effect.payload.type));
  const canAct = Boolean(interactive && !pending && !terminal);
  const ownSeat = view.private_view.status === "seat" ? view.private_view.seat : null;
  const opponentSeat = ownSeat === "seat_0" ? "seat_1" : ownSeat === "seat_1" ? "seat_0" : null;
  const outcomeExplanation = terminal
    ? outcomeSurfaceData({
        gameId: "masked_claims",
        heading: terminalLabel(view),
        rationale: view.terminal_rationale ?? null,
        resultKind: view.terminal.draw ? "draw" : "win",
        decisiveCause: view.terminal.kind,
        templateKey: templateKey(view),
        templateParams: { winner: view.terminal.winner ?? "", tiebreak: "tiebreak" in view.terminal ? view.terminal.tiebreak : "" },
        finalStanding: [maskedStanding("seat_0", view), maskedStanding("seat_1", view)],
        breakdownSections: [
          {
            id: "scores",
            heading: "Scores",
            rows: [
              { label: "seat_0 score", value: view.scores.seat_0 },
              { label: "seat_1 score", value: view.scores.seat_1 },
            ],
          },
          {
            id: "claims",
            heading: "Claims",
            rows: [
              { label: "seat_0 veiled claims", value: view.veiled_gallery[0].length },
              { label: "seat_1 veiled claims", value: view.veiled_gallery[1].length },
              { label: "seat_0 exposed masks", value: view.exposed_rows[0].length },
              { label: "seat_1 exposed masks", value: view.exposed_rows[1].length },
            ],
          },
        ],
      })
    : null;

  return (
    <section
      className={`plain-tricks-board masked-claims-board ${terminal ? "terminal" : ""}${revealActive ? " reveal" : ""}${
        reducedMotion ? " reduced" : ""
      }`}
      aria-labelledby="masked-claims-heading"
      data-testid="masked-claims-board"
    >
      <div className="plain-tricks-banner">
        <div>
          <p className="eyebrow">{view.display_name}</p>
          <h2 id="masked-claims-heading">{statusLabel(view)}</h2>
        </div>
        <span className="turn-pill" data-testid="turn">
          {terminalLabel(view)}
        </span>
      </div>

      <p className="sr-only" aria-live="polite">
        {boardSummary(view, claimChoices.length, responseChoices.length)}
      </p>

      <div className="plain-tricks-metrics" aria-label="Masked Claims status">
        <Metric label="Turn" value={`${view.turn_index + 1} / 8`} />
        <Metric label="Claimant" value={seatLabel(view.claimant)} />
        <Metric label="Scores" value={`${view.scores.seat_0} - ${view.scores.seat_1}`} />
        <Metric label="Active" value={view.active_seat ? seatLabel(view.active_seat) : "Terminal"} />
      </div>

      <div className="plain-tricks-table" aria-label="Masked Claims table">
        <section className="plain-seat active" aria-label={ownSeat ? `${seatLabel(ownSeat)} private masks` : "Observer mask state"}>
          <div className="plain-section-heading">
            <span>Private hand</span>
            <strong>{ownSeat ? `${seatLabel(ownSeat)} view` : "Observer"}</strong>
          </div>
          {view.private_view.status === "seat" ? (
            <div className="plain-hand" data-testid="masked-claims-own-hand">
              {canAct && claimChoices.length > 0 ? (
                <p className="masked-claim-legend" data-testid="masked-claims-legend">
                  Each declaration is tagged against this mask&apos;s true grade:{" "}
                  <span className="masked-claim-cue true">true</span> exact,{" "}
                  <span className="masked-claim-cue under">under</span> safe underclaim,{" "}
                  <span className="masked-claim-cue bluff">bluff</span> overclaim (loses if challenged).
                </p>
              ) : null}
              {view.private_view.own_hand.map((mask, index) => {
                const choice = legalClaims.get(mask.tile_id) ?? null;
                return (
                  <MaskClaimCard
                    key={`${index}-${mask.grade}`}
                    mask={mask}
                    index={index}
                    turnIndex={view.turn_index}
                    choice={choice}
                    gradeLabels={view.ui.grade_labels}
                    disabled={!canAct || !choice}
                    onPathSubmit={onPathSubmit}
                  />
                );
              })}
            </div>
          ) : (
            <FaceDownCount count={view.hand_counts.seat_0 + view.hand_counts.seat_1} label="Masks hidden" testId="masked-claims-observer-hand" />
          )}
        </section>

        <section className="plain-trick-surface" aria-label="Pending claim">
          <div className="plain-section-heading">
            <span>Pedestal</span>
            <strong>{view.pedestal ? `${seatLabel(view.pedestal.claimant)} claimed` : "No pending claim"}</strong>
          </div>
          <div className="plain-played-row">
            {view.pedestal ? (
              <div className="plain-played-card">
                <span>{seatLabel(view.pedestal.claimant)}</span>
                <strong>{view.pedestal.declared_label}</strong>
                <small>Pending response</small>
              </div>
            ) : (
              <p className="muted">Waiting for a claimant to place a mask.</p>
            )}
          </div>

          {responseChoices.length > 0 || view.phase.includes("reaction") ? (
            <div className="plain-latest" role="status" data-testid="masked-claims-response-state">
              <span>{responseChoices.length > 0 ? "Response" : "Waiting"}</span>
              <strong>{responsePrompt(view, responseChoices.length)}</strong>
              {responseChoices.length > 0 ? (
                <div className="action-list">
                  {responseChoices.map((choice, index) => (
                    <button
                      type="button"
                      key={choice.segment}
                      disabled={!canAct}
                      aria-label={choice.accessibility_label}
                      data-testid={`masked-claims-response-turn-${view.turn_index}-${index}`}
                      onClick={() => onPathSubmit?.([choice.segment])}
                    >
                      {choice.label}
                    </button>
                  ))}
                </div>
              ) : null}
            </div>
          ) : null}
        </section>

        <section className="plain-seat opponent" aria-label="Opposing mask count">
          <div className="plain-section-heading">
            <span>Opposing hand</span>
            <strong>{opponentSeat ? seatLabel(opponentSeat) : "Both seats"}</strong>
          </div>
          <FaceDownCount
            count={opponentSeat ? view.hand_counts[opponentSeat] : view.hand_counts.seat_0 + view.hand_counts.seat_1}
            label="Masks hidden"
            testId="masked-claims-opponent-hand"
          />
        </section>
      </div>

      <section className="plain-history" aria-label="Veiled claim gallery">
        <div className="plain-section-heading">
          <span>Veiled claims</span>
          <strong>{view.veiled_gallery[0].length + view.veiled_gallery[1].length} accepted</strong>
        </div>
        <ol>
          {view.veiled_gallery.map((claims, seatIndex) =>
            claims.map((claim, index) => (
              <li key={`veiled-${seatIndex}-${index}`} data-testid={`masked-claims-veiled-${seatIndex}-${index}`}>
                <span>{seatLabel(seatFromIndex(seatIndex))}</span>
                <strong>{claim.declared_label}</strong>
                <small>Veiled</small>
              </li>
            )),
          )}
        </ol>
      </section>

      <section className="plain-history" aria-label="Exposed masks">
        <div className="plain-section-heading">
          <span>Exposed masks</span>
          <strong>{view.exposed_rows[0].length + view.exposed_rows[1].length} revealed</strong>
        </div>
        {view.exposed_rows[0].length + view.exposed_rows[1].length === 0 ? (
          <p className="muted">Challenges reveal masks here.</p>
        ) : (
          <ol>
            {view.exposed_rows.map((row, seatIndex) =>
              row.map((mask, index) => (
                <li key={`exposed-${seatIndex}-${index}`} data-testid={`masked-claims-exposed-${seatIndex}-${index}`}>
                  <span>{seatLabel(mask.claimant)}</span>
                  <strong>
                    {gradeLabel(mask.actual_grade, view)} vs {gradeLabel(mask.declared_grade, view)}
                  </strong>
                  <small>Challenged by {seatLabel(mask.challenger)}</small>
                </li>
              )),
            )}
          </ol>
        )}
      </section>

      <section className="plain-history" aria-label="Claim counters">
        <div className="plain-section-heading">
          <span>Counters</span>
          <strong>Challenge record</strong>
        </div>
        <ol>
          {view.counters.map((counter, index) => (
            <li key={`counter-${index}`}>
              <span>{seatLabel(seatFromIndex(index))}</span>
              <strong>{counter.successful_challenges} successful</strong>
              <small>{counter.exposed_lies} exposed lies, {counter.challenges_declared} challenges</small>
            </li>
          ))}
        </ol>
      </section>

      <div className="plain-latest" role="status">
        <span>{outcomeExplanation ? "Outcome" : feedback?.title ?? "Waiting"}</span>
        <strong>
          {outcomeExplanation
            ? outcomeAnnouncementText(outcomeExplanation)
            : feedback?.detail ?? "Legal claims, responses, and hidden mask views will update here."}
        </strong>
      </div>

      {outcomeExplanation ? <OutcomeExplanationPanel reducedMotion={reducedMotion} explanation={outcomeExplanation} /> : null}
    </section>
  );
}

function MaskClaimCard({
  mask,
  index,
  turnIndex,
  choice,
  gradeLabels,
  disabled,
  onPathSubmit,
}: {
  mask: MaskedClaimsMaskView;
  index: number;
  turnIndex: number;
  choice: ActionChoice | null;
  gradeLabels: string[];
  disabled: boolean;
  onPathSubmit?: (path: string[]) => void;
}) {
  const declaredChoices = choice?.next?.choices ?? [];
  const actualRank = gradeRank(mask.grade, gradeLabels);
  const total = gradeLabels.length;
  return (
    <article className={`plain-card ${choice ? "legal" : ""}`}>
      <span>Held mask</span>
      <strong>{mask.label}</strong>
      {actualRank ? (
        <small className="masked-grade-badge" data-testid={`masked-claims-grade-${turnIndex}-${index}`}>
          True grade {actualRank} of {total}
        </small>
      ) : null}
      <small>{choice ? "Choose declared grade" : "Held"}</small>
      {declaredChoices.length > 0 ? (
        <div className="action-list">
          {declaredChoices.map((declared, declaredIndex) => {
            const relation = claimRelation(declaredIndex + 1, actualRank);
            return (
              <button
                type="button"
                key={declared.segment}
                className={relation ? `masked-declare ${relation}` : "masked-declare"}
                disabled={disabled}
                aria-label={declared.accessibility_label}
                data-testid={`masked-claims-claim-turn-${turnIndex}-${index}-${declaredIndex}`}
                onClick={() => choice && onPathSubmit?.(["claim", choice.segment, declared.segment])}
              >
                <span>{declared.label}</span>
                {relation ? <span className={`masked-claim-cue ${relation}`}>{relation}</span> : null}
              </button>
            );
          })}
        </div>
      ) : null}
    </article>
  );
}

function gradeRank(grade: string, gradeLabels: string[]): number | null {
  // Rust serializes a mask's own grade as a 1-based numeric string ("1".."5").
  const numeric = Number(grade);
  if (Number.isInteger(numeric) && numeric >= 1 && numeric <= gradeLabels.length) {
    return numeric;
  }
  // Fall back to matching a grade label (defensive; not expected in current views).
  const idx = gradeLabels.findIndex((entry) => entry.toLowerCase().startsWith(grade.toLowerCase()));
  return idx >= 0 ? idx + 1 : null;
}

function claimRelation(declaredRank: number | null, actualRank: number | null): "true" | "under" | "bluff" | null {
  if (declaredRank === null || actualRank === null) {
    return null;
  }
  if (declaredRank > actualRank) {
    return "bluff";
  }
  if (declaredRank === actualRank) {
    return "true";
  }
  return "under";
}

function claimMaskChoices(actionTree: ActionTree | null): ActionChoice[] {
  return actionTree?.choices.find((choice) => choice.segment === "claim")?.next?.choices ?? [];
}

function responseActionChoices(actionTree: ActionTree | null): ActionChoice[] {
  return actionTree?.choices.filter((choice) => choice.tags?.includes("respond")) ?? [];
}

function FaceDownCount({ count, label, testId }: { count: number; label: string; testId: string }) {
  return (
    <div className="plain-facedown" data-testid={testId}>
      <span>{label}</span>
      <strong>{count}</strong>
      <small>{count === 1 ? "mask" : "masks"}</small>
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function maskedStanding(seat: SeatId, view: MaskedClaimsPublicView) {
  const index = seat === "seat_0" ? 0 : 1;
  return {
    id: seat,
    label: seatLabel(seat),
    result: view.terminal.draw ? "draw" : view.terminal.winner === seat ? "win" : "loss",
    emphasized: view.terminal.winner === seat,
    values: [
      { label: "Score", value: view.scores[seat] },
      { label: "Successful challenges", value: view.counters[index].successful_challenges },
      { label: "Exposed lies", value: view.counters[index].exposed_lies },
    ],
  };
}

function isRevealEffect(type: string): boolean {
  return type === "mask_revealed" || type === "challenge_resolved" || type === "terminal";
}

function templateKey(view: MaskedClaimsPublicView): string {
  if (view.terminal.kind === "draw") {
    return "masked_claims.draw";
  }
  if (view.terminal.kind === "tiebreak_win") {
    if (view.terminal.tiebreak === "fewer_exposed_lies") {
      return "masked_claims.tiebreak_exposed_lies";
    }
    if (view.terminal.tiebreak === "more_successful_challenges") {
      return "masked_claims.tiebreak_successful_challenges";
    }
    if (view.terminal.tiebreak === "fewer_challenges_declared") {
      return "masked_claims.tiebreak_challenges_declared";
    }
    return "masked_claims.tiebreak_win";
  }
  return "masked_claims.score_win";
}

function statusLabel(view: MaskedClaimsPublicView): string {
  if (view.terminal.kind === "draw") {
    return "Draw";
  }
  if (view.terminal.kind !== "non_terminal") {
    return `${seatLabel(view.terminal.winner)} wins`;
  }
  if (view.phase.includes("reaction")) {
    return `${seatLabel(view.active_seat ?? view.claimant)} to respond`;
  }
  return `${seatLabel(view.claimant)} to claim`;
}

function terminalLabel(view: MaskedClaimsPublicView): string {
  if (view.terminal.kind === "draw") {
    return "Draw";
  }
  if (view.terminal.kind !== "non_terminal") {
    return `${view.terminal.winner} won`;
  }
  return view.active_seat ?? "Terminal";
}

function responsePrompt(view: MaskedClaimsPublicView, responseCount: number): string {
  if (responseCount > 0) {
    const declared = view.pedestal?.declared_label ?? "claim";
    return `${seatLabel(view.active_seat ?? view.claimant)} may accept or challenge ${declared}.`;
  }
  if (view.pedestal) {
    return `${seatLabel(view.pedestal.claimant)} is waiting for a response.`;
  }
  return "Waiting for a response window.";
}

function boardSummary(view: MaskedClaimsPublicView, claimCount: number, responseCount: number): string {
  return `${view.display_name}, ${statusLabel(view)}, turn ${view.turn_index + 1}, ${claimCount} claim choices, ${responseCount} response choices.`;
}

function gradeLabel(grade: string, view: MaskedClaimsPublicView): string {
  return view.ui.grade_labels.find((label) => label.toLowerCase().startsWith(grade.toLowerCase())) ?? grade;
}

function seatFromIndex(index: number): SeatId {
  return index === 0 ? "seat_0" : "seat_1";
}

function seatLabel(seat: SeatId | null): string {
  if (!seat) {
    return "Terminal";
  }
  return resolveSeatLabel(seat);
}

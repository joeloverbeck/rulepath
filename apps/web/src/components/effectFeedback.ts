import { useEffect, useState } from "react";
import type { EffectEntry } from "../wasm/client";

const STORAGE_KEY = "rulepath.reducedMotion";

export type ReducedMotionOverride = "system" | "reduce" | "motion";

export type EffectFeedback = {
  title: string;
  detail: string;
  tone: "neutral" | "movement" | "turn" | "terminal";
};

export function feedbackForEffect(entry: EffectEntry): EffectFeedback {
  const payload = entry.effect.payload;
  switch (payload.type) {
    case "action_started":
      return {
        title: "Action started",
        detail: `${payload.actor} chose a committed action.`,
        tone: "neutral",
      };
    case "counter_advanced":
      return {
        title: "Counter advanced",
        detail: `${payload.actor} moved from ${payload.from} to ${payload.to}.`,
        tone: "movement",
      };
    case "turn_changed":
      return {
        title: "Turn changed",
        detail: `${payload.next_actor} is now active.`,
        tone: "turn",
      };
    case "game_ended":
      return {
        title: "Game ended",
        detail: payload.winner ? `${payload.winner} won the match.` : "The match ended.",
        tone: "terminal",
      };
    case "mark_placed":
      return {
        title: "Mark placed",
        detail: `${payload.seat} placed on ${payload.cell}.`,
        tone: "movement",
      };
    case "drop_accepted":
      return {
        title: "Drop accepted",
        detail: `${payload.seat} chose ${payload.column}.`,
        tone: "neutral",
      };
    case "piece_landed":
      return {
        title: "Piece landed",
        detail: `${payload.seat} landed on ${payload.cell}.`,
        tone: "movement",
      };
    case "placement_accepted":
      return {
        title: "Placement accepted",
        detail: `${payload.seat} chose ${payload.cell}.`,
        tone: "neutral",
      };
    case "disc_placed":
      return {
        title: "Disc placed",
        detail: `${payload.seat} placed on ${payload.cell}.`,
        tone: "movement",
      };
    case "discs_flipped":
      return {
        title: "Discs flipped",
        detail: `${payload.seat} flipped ${flipCount(payload.flips)} disc${flipCount(payload.flips) === 1 ? "" : "s"}.`,
        tone: "movement",
      };
    case "pass_taken":
      return {
        title: "Pass taken",
        detail: `${payload.seat} had no legal placement.`,
        tone: "turn",
      };
    case "active_player_changed":
      return {
        title: "Turn changed",
        detail: `${payload.active_seat} is now active.`,
        tone: "turn",
      };
    case "line_completed":
      return {
        title: "Line completed",
        detail: `${payload.winning_seat} completed a line.`,
        tone: "terminal",
      };
    case "win_detected":
      return {
        title: "Win detected",
        detail: `${payload.winning_seat} completed a line.`,
        tone: "terminal",
      };
    case "draw_reached":
    case "draw_detected":
      return {
        title: "Draw reached",
        detail: "The board is full without a winner.",
        tone: "terminal",
      };
    case "bot_chose_action":
      return {
        title: "Bot chose action",
        detail: `${payload.policy_id} selected ${payload.action_id ?? formatPath(payload.action_path)}.`,
        tone: "neutral",
      };
    case "move_committed":
      return {
        title: "Move committed",
        detail: `${payload.seat} moved ${payload.piece_id} from ${payload.start_cell} to ${payload.final_cell}.`,
        tone: "movement",
      };
    case "quiet_step":
      return {
        title: "Quiet step",
        detail: `${payload.piece_id} moved from ${payload.origin} to ${payload.landing}.`,
        tone: "movement",
      };
    case "capture_step":
      return {
        title: "Capture step",
        detail: `${payload.piece_id} landed on ${payload.landing} and captured ${payload.captured_piece_id}.`,
        tone: "movement",
      };
    case "promotion":
      return {
        title: "Promotion",
        detail: `${payload.piece_id} promoted on ${payload.cell}.`,
        tone: "movement",
      };
    case "forced_capture_available":
      return {
        title: "Forced capture",
        detail: String(payload.explanation ?? "A capture is available."),
        tone: "turn",
      };
    case "forced_continuation_required":
      return {
        title: "Forced continuation",
        detail: String(payload.explanation ?? "The capture path must continue."),
        tone: "turn",
      };
    case "commit_face_down":
      return {
        title: "Commitment placed",
        detail: `${payload.seat} committed a card face-down.`,
        tone: "neutral",
      };
    case "own_commit_confirmed":
      return {
        title: "Private commitment confirmed",
        detail: "Your selected card was committed face-down.",
        tone: "neutral",
      };
    case "cards_revealed":
      return {
        title: "Cards revealed",
        detail: "Rust revealed both committed cards.",
        tone: "movement",
      };
    case "round_scored":
      return {
        title: "Round scored",
        detail:
          "garrison_points" in payload || "prospector_points" in payload
            ? `Rust scored round ${payload.round ?? "current"}: Garrison +${payload.garrison_points ?? 0}, Prospectors +${
                payload.prospector_points ?? 0
              }.`
            : "round_counts" in payload
            ? "Rust updated round and match trick totals."
            : payload.winner
              ? `${payload.winner} won the round.`
              : "The round was drawn.",
        tone: "turn",
      };
    case "commitment_placed":
      return {
        title: "Commitment placed",
        detail: `${payload.seat} committed a hidden draft choice.`,
        tone: "neutral",
      };
    case "own_commit_accepted":
      return {
        title: "Commitment accepted",
        detail: "Your hidden draft choice was recorded.",
        tone: "neutral",
      };
    case "pending_seats_changed":
      return {
        title: "Pending seats updated",
        detail: `seat_0 ${payload.seat_0_committed ? "committed" : "waiting"}, seat_1 ${
          payload.seat_1_committed ? "committed" : "waiting"
        }.`,
        tone: "turn",
      };
    case "reveal_batch_started":
      return {
        title: "Reveal batch",
        detail: "Rust started the grouped reveal.",
        tone: "movement",
      };
    case "choices_revealed":
      return {
        title: "Choices revealed",
        detail: "Both hidden draft choices are now public.",
        tone: "movement",
      };
    case "draft_resolved":
      return {
        title: "Draft resolved",
        detail: "Rust awarded the revealed draft items.",
        tone: "movement",
      };
    case "pool_changed":
      return {
        title: "Pool changed",
        detail: `${payload.remaining_count} public items remain.`,
        tone: "turn",
      };
    case "score_changed":
      return {
        title: "Score changed",
        detail: "Secret Draft scores were updated by Rust.",
        tone: "turn",
      };
    case "claim_score_changed":
      return {
        title: "Score changed",
        detail: `${payload.seat} now holds ${payload.total} (+${payload.delta}).`,
        tone: "turn",
      };
    case "round_advanced":
      return {
        title: "Round advanced",
        detail: `${payload.priority_seat} has conflict priority next round.`,
        tone: "turn",
      };
    case "crest_deal_started":
      return {
        title: "Crests prepared",
        detail: "Rust prepared private and center crests.",
        tone: "neutral",
      };
    case "opening_pool_set":
      return {
        title: "Shared pool set",
        detail: `Shared pool ${payload.shared_pool ?? 0}.`,
        tone: "turn",
      };
    case "pledge_held":
      return {
        title: "Held",
        detail: `${payload.actor} held without adding to the shared pool.`,
        tone: "turn",
      };
    case "pledge_pressed":
      return {
        title: "Pressed",
        detail: `${payload.actor} added ${payload.amount ?? 0} to the shared pool.`,
        tone: "turn",
      };
    case "pledge_lifted":
      return {
        title: "Lifted",
        detail: `${payload.actor} lifted the pledge by ${payload.amount ?? 0}.`,
        tone: "turn",
      };
    case "pledge_matched":
      return {
        title: "Matched",
        detail: `${payload.actor} matched ${payload.amount ?? 0}.`,
        tone: "turn",
      };
    case "seat_yielded":
      return {
        title: "Yielded",
        detail: `${payload.actor} yielded; ${payload.winner} receives the shared pool.`,
        tone: "terminal",
      };
    case "center_reveal_started":
      return {
        title: "Center reveal",
        detail: "Rust started the grouped center reveal.",
        tone: "movement",
      };
    case "center_revealed":
      return {
        title: "Center revealed",
        detail: "The center crest is now public.",
        tone: "movement",
      };
    case "showdown_reveal_started":
      return {
        title: "Showdown reveal",
        detail: "Rust started the grouped showdown reveal.",
        tone: "movement",
      };
    case "showdown_revealed":
      return {
        title: "Showdown revealed",
        detail: "Both private crests are now public.",
        tone: "movement",
      };
    case "ledger_resolved":
      return {
        title: "Ledger resolved",
        detail: `Shared pool ${payload.shared_pool ?? 0} resolved by Rust.`,
        tone: "terminal",
      };
    case "river_ledger_contribution_changed":
      return {
        title: "Ledger updated",
        detail: `${payload.actor} added ${payload.amount_added ?? 0}; ledger total ${payload.pot_total ?? 0}.`,
        tone: "turn",
      };
    case "river_ledger_stack_changed":
      return {
        title: "Stack updated",
        detail: `${payload.seat ?? payload.actor ?? "Seat"} has ${payload.remaining_stack ?? 0} remaining.`,
        tone: "turn",
      };
    case "river_ledger_seat_became_all_in":
      return {
        title: "All-in",
        detail: `${payload.seat ?? payload.actor ?? "Seat"} is all-in.`,
        tone: "turn",
      };
    case "river_ledger_uncalled_contribution_returned":
      return {
        title: "Uncalled returned",
        detail: `${payload.seat ?? "Seat"} receives ${payload.amount ?? 0}.`,
        tone: "turn",
      };
    case "river_ledger_pot_resolved":
      return {
        title: "Pot resolved",
        detail: `Rust resolved ${payload.amount ?? payload.pot_amount ?? 0} from ${riverPotLabel(payload.pot_id)}.`,
        tone: "terminal",
      };
    case "river_ledger_pot_awarded":
      return {
        title: "Pot awarded",
        detail: `${payload.seat ?? payload.winner ?? "Winner"} receives ${payload.amount ?? 0} from ${riverPotLabel(payload.pot_id)}.`,
        tone: "terminal",
      };
    case "river_ledger_street_advanced":
      return {
        title: "Board revealed",
        detail: `${riverStreetLabel(payload.street)} is active with ${payload.public_board_count ?? 0} public cards.`,
        tone: "movement",
      };
    case "river_ledger_showdown_resolved":
      return {
        title: "Showdown resolved",
        detail:
          payload.kind === "last_live_hand"
            ? `Last live hand receives ledger total ${payload.pot_total ?? 0}.`
            : `Showdown settled ${payload.winner_count ?? 0} winner(s) from ledger total ${payload.pot_total ?? 0}.`,
        tone: "terminal",
      };
    case "deal_started":
      return {
        title: "Deal started",
        detail: `Rust prepared ${payload.cards_per_seat ?? 0} cards per seat.`,
        tone: "neutral",
      };
    case "hand_dealt":
      return {
        title: "Hand dealt",
        detail: "Your private hand was dealt by Rust.",
        tone: "neutral",
      };
    case "deal_completed":
      return {
        title: "Deal completed",
        detail: `${payload.leader} leads this deal.`,
        tone: "turn",
      };
    case "card_played":
      return {
        title: "Card played",
        detail: payload.summary ? String(payload.summary) : `${payload.seat} played a public card.`,
        tone: "movement",
      };
    case "bid_accepted":
      return {
        title: "Bid accepted",
        detail: String(payload.summary ?? "Rust recorded the public bid."),
        tone: "turn",
      };
    case "dealer_hook_constrained":
      return {
        title: "Dealer hook applied",
        detail: String(payload.summary ?? "Rust removed the hooked dealer bid."),
        tone: "turn",
      };
    case "bidding_completed":
      return {
        title: "Bidding complete",
        detail: String(payload.summary ?? "Rust moved from bidding to trick play."),
        tone: "turn",
      };
    case "trick_captured":
      return {
        title: "Trick captured",
        detail: String(payload.summary ?? "Rust awarded the trick."),
        tone: "movement",
      };
    case "hand_scored":
      return {
        title: "Hand scored",
        detail: String(payload.summary ?? "Rust scored exact bids."),
        tone: "turn",
      };
    case "hand_advanced":
      return {
        title: "Hand advanced",
        detail: String(payload.summary ?? "Rust dealt the next scheduled hand."),
        tone: "turn",
      };
    case "match_completed":
      return {
        title: "Match completed",
        detail: String(payload.summary ?? "Rust finalized the match standings."),
        tone: "terminal",
      };
    case "trick_resolved":
      return {
        title: "Trick resolved",
        detail: `${payload.winner} won the trick.`,
        tone: "turn",
      };
    case "deal_rotated":
      return {
        title: "Deal rotated",
        detail: `${payload.leader} leads the next deal.`,
        tone: "turn",
      };
    case "match_resolved":
      return {
        title: "Match resolved",
        detail: "Rust resolved final trick totals.",
        tone: "terminal",
      };
    case "claim_placed":
      return {
        title: "Claim placed",
        detail: `${payload.claimant} claimed ${payload.declared_grade}.`,
        tone: "neutral",
      };
    case "reaction_window_opened":
      return {
        title: "Response window",
        detail: `${payload.responder} may accept or challenge.`,
        tone: "turn",
      };
    case "claim_accepted":
      return {
        title: "Claim accepted",
        detail: `${payload.claimant} scored ${payload.score_delta ?? 0}; the mask stays veiled.`,
        tone: "turn",
      };
    case "challenge_declared":
      return {
        title: "Challenge declared",
        detail: `${payload.responder} challenged the claim.`,
        tone: "movement",
      };
    case "mask_revealed":
      return {
        title: "Mask revealed",
        detail: `Rust revealed a challenged ${payload.actual_grade} mask.`,
        tone: "movement",
      };
    case "challenge_resolved":
      return {
        title: "Challenge resolved",
        detail: `Rust resolved the challenge as ${payload.outcome}.`,
        tone: "turn",
      };
    case "bot_chose_action_public":
      return {
        title: "Bot chose action",
        detail: `${payload.policy_id} selected ${payload.action_family}.`,
        tone: "neutral",
      };
    case "resource_collected":
      return {
        title: "Resources collected",
        detail: `${payload.seat} collected ${resourceCounts(payload.gain)}.`,
        tone: "movement",
      };
    case "resource_exchanged":
      return {
        title: "Resources exchanged",
        detail: `${payload.seat} paid ${resourceCounts(payload.cost)} and gained ${resourceCounts(payload.gain)}.`,
        tone: "movement",
      };
    case "contract_fulfilled":
      return {
        title: "Contract fulfilled",
        detail: `${payload.seat} fulfilled ${payload.contract} for ${payload.points} points.`,
        tone: "turn",
      };
    case "slot_refilled":
      return {
        title: "Market refilled",
        detail: `${payload.slot} refilled with ${payload.contract}.`,
        tone: "movement",
      };
    case "slot_emptied":
      return {
        title: "Market slot emptied",
        detail: `${payload.slot} is empty with ${payload.remaining_queue_len} queued.`,
        tone: "turn",
      };
    case "pass_accepted":
      return {
        title: "Pass accepted",
        detail: `${payload.seat} had no economy action available.`,
        tone: "turn",
      };
    case "turn_advanced":
      return {
        title: "Turn advanced",
        detail: `${payload.active_seat} is now active.`,
        tone: "turn",
      };
    case "claim_turn_advanced":
      return {
        title: "Turn advanced",
        detail: `${payload.claimant} is now active.`,
        tone: "turn",
      };
    case "district_bailed":
      return {
        title: "District bailed",
        detail: `${payload.district} flood level fell by ${payload.amount ?? 0}.`,
        tone: "movement",
      };
    case "levee_placed":
      return {
        title: "Levee placed",
        detail: `${payload.district} gained ${payload.amount ?? 0} levee${payload.amount === 1 ? "" : "s"}.`,
        tone: "movement",
      };
    case "forecast_revealed":
      return {
        title: "Forecast revealed",
        detail: `The next public storm card is ${cardLabel(payload.card)}.`,
        tone: "turn",
      };
    case "environment_phase_began":
      return {
        title: "Storm phase",
        detail: `Rust started turn ${payload.turn ?? "current"} storm resolution.`,
        tone: "turn",
      };
    case "event_drawn":
      return {
        title: "Storm card drawn",
        detail: `Storm card ${payload.index ?? "next"} was revealed as ${cardLabel(payload.card)}.`,
        tone: "movement",
      };
    case "levee_absorbed":
      return {
        title: "Levee absorbed flood",
        detail: `${payload.district} spent ${payload.amount ?? 0} levee; ${payload.remaining_levees ?? 0} remain.`,
        tone: "movement",
      };
    case "flood_level_rose":
      return {
        title: "Flood rose",
        detail: `${payload.district} rose by ${payload.amount ?? 0} to ${payload.new_level ?? 0}.`,
        tone: "movement",
      };
    case "district_inundated":
      return {
        title: "District inundated",
        detail: `${payload.district} reached the inundation threshold.`,
        tone: "terminal",
      };
    case "deck_exhausted":
      return {
        title: "Storm deck exhausted",
        detail: "The team survived every public storm card.",
        tone: "terminal",
      };
    case "crew_marched":
      return {
        title: "Crew marched",
        detail: `Prospectors moved from ${siteLabel(payload.from)} to ${siteLabel(payload.to)}.`,
        tone: "movement",
      };
    case "guard_patrolled":
      return {
        title: "Guard patrolled",
        detail: `Garrison moved from ${siteLabel(payload.from)} to ${siteLabel(payload.to)}.`,
        tone: "movement",
      };
    case "clash_resolved":
      return {
        title: "Clash resolved",
        detail: `${siteLabel(payload.site)} resolved for ${factionLabel(payload.entering_faction)}.`,
        tone: "movement",
      };
    case "stake_placed":
      return {
        title: "Stake placed",
        detail: `Prospectors placed a stake at ${siteLabel(payload.site)}.`,
        tone: "movement",
      };
    case "stake_dismantled":
      return {
        title: "Stake dismantled",
        detail: `Garrison dismantled the stake at ${siteLabel(payload.site)}.`,
        tone: "movement",
      };
    case "crew_mustered":
      return {
        title: "Crew mustered",
        detail: `Prospectors mustered at ${siteLabel(payload.site)}; crews now ${payload.crews ?? 0}.`,
        tone: "movement",
      };
    case "guard_reinforced":
      return {
        title: "Guard reinforced",
        detail: `Garrison reinforced ${siteLabel(payload.site)}; guards now ${payload.guards ?? 0}.`,
        tone: "movement",
      };
    case "event_resolved":
      return {
        title: "Event resolved",
        detail: `${cardLabel(payload.card)} resolved: ${String(payload.summary ?? "public event effect")}.`,
        tone: "movement",
      };
    case "edict_activated":
      return {
        title: "Edict activated",
        detail: `${cardLabel(payload.card)} activated ${String(payload.edict ?? "an edict")}.`,
        tone: "turn",
      };
    case "edict_expired":
      return {
        title: "Edict expired",
        detail: `${String(payload.edict ?? "An edict")} expired at Reckoning.`,
        tone: "turn",
      };
    case "card_revealed":
      return {
        title: "Card revealed",
        detail: `${cardLabel(payload.card)} is current; next public card is ${cardLabel(payload.next_public)}.`,
        tone: "movement",
      };
    case "choice_taken":
      return {
        title: "Choice taken",
        detail: `${factionLabel(payload.faction)} chose ${String(payload.choice ?? "an action")}.`,
        tone: "turn",
      };
    case "card_discarded":
      return {
        title: "Card discarded",
        detail: `${cardLabel(payload.card)} was discarded: ${String(payload.reason ?? "resolved")}.`,
        tone: "turn",
      };
    case "eligibility_changed":
      return {
        title: "Eligibility changed",
        detail: `${factionLabel(payload.faction)} is ${payload.eligible ? "eligible" : "ineligible"}: ${String(payload.reason ?? "Rust updated eligibility")}.`,
        tone: "turn",
      };
    case "resources_changed":
      return {
        title: "Resources changed",
        detail: `${factionLabel(payload.faction)} resource changed from ${payload.previous ?? 0} to ${payload.new ?? 0}.`,
        tone: "turn",
      };
    case "op_resolved":
      return {
        title: "Operation resolved",
        detail: `${factionLabel(payload.faction)} resolved ${String(payload.op ?? "operation")} on ${formatSites(payload.sites)}.`,
        tone: "movement",
      };
    case "agent_placed":
      return {
        title: "Agent placed",
        detail: `${siteLabel(payload.site)} now has ${payload.new_count ?? 0} agents.`,
        tone: "movement",
      };
    case "agent_removed":
      return {
        title: "Agent removed",
        detail: `${siteLabel(payload.site)} now has ${payload.new_count ?? 0} agents.`,
        tone: "movement",
      };
    case "depot_built":
      return {
        title: "Depot built",
        detail: `${siteLabel(payload.site)} gained a depot.`,
        tone: "movement",
      };
    case "cache_removed":
      return {
        title: "Cache removed",
        detail: `${siteLabel(payload.site)} now has ${payload.new_count ?? 0} caches.`,
        tone: "movement",
      };
    case "settler_moved":
      return {
        title: "Settler moved",
        detail: `Settler moved from ${siteLabel(payload.from)} to ${siteLabel(payload.to)}.`,
        tone: "movement",
      };
    case "cache_laid":
      return {
        title: "Cache laid",
        detail: `${siteLabel(payload.site)} now has ${payload.new_count ?? 0} caches.`,
        tone: "movement",
      };
    case "settler_rallied":
      return {
        title: "Settler rallied",
        detail: `${siteLabel(payload.site)} now has ${payload.new_count ?? 0} settlers.`,
        tone: "movement",
      };
    case "reckoning_resolved":
      return {
        title: "Reckoning resolved",
        detail: `Reckoning ${payload.round ?? "current"} resolved: ${reckoningResult(payload.victory_check)}.`,
        tone: "turn",
      };
    case "turn_ended":
      return {
        title: "Turn ended",
        detail: `${factionLabel(payload.faction)} ended round ${payload.round ?? "current"}.`,
        tone: "turn",
      };
    case "refill_started":
      return {
        title: "Next round",
        detail: `${payload.next_lead_seat} leads the next round.`,
        tone: "turn",
      };
    case "terminal":
      if (isMaskedClaimsOutcome(payload.outcome)) {
        return {
          title: "Claims complete",
          detail: terminalOutcome(payload.outcome, "claim ledger"),
          tone: "terminal",
        };
      }
      if (typeof payload.outcome === "string" && "summary" in payload) {
        return {
          title: "Flood Watch complete",
          detail: payload.outcome === "won" ? "The team survived the storm deck." : "The team lost to inundation.",
          tone: "terminal",
        };
      }
      if ("garrison_total" in payload || "prospector_total" in payload) {
        return {
          title: "Frontier complete",
          detail: `${factionLabel(payload.winner)} wins ${payload.garrison_total ?? 0}-${payload.prospector_total ?? 0}.`,
          tone: "terminal",
        };
      }
      if ("victory_type" in payload && "totals" in payload) {
        return {
          title: "Event Frontier complete",
          detail: `${factionLabel(payload.winner)} won by ${String(payload.victory_type)}.`,
          tone: "terminal",
        };
      }
      if (isPlainTricksOutcome(payload.outcome)) {
        return {
          title: "Tricks complete",
          detail: terminalOutcome(payload.outcome, "match"),
          tone: "terminal",
        };
      }
      if ("final_scores" in payload) {
        return {
          title: "Draft complete",
          detail: terminalOutcome(payload.outcome, "draft"),
          tone: "terminal",
        };
      }
      if (isPokerLiteOutcome(payload.outcome)) {
        return {
          title: "Ledger complete",
          detail: terminalOutcome(payload.outcome, "ledger"),
          tone: "terminal",
        };
      }
      if ("outcome" in payload) {
        return {
          title: "Bazaar complete",
          detail: terminalOutcome(payload.outcome, "bazaar"),
          tone: "terminal",
        };
      }
      return {
        title: "Duel complete",
        detail: payload.winner ? `${payload.winner} won the duel.` : "The duel ended in a draw.",
        tone: "terminal",
      };
    case "placement_rejected":
      return {
        title: "Placement rejected",
        detail: String(payload.label ?? payload.reason ?? "Rejected"),
        tone: "neutral",
      };
    case "action_completed":
      return {
        title: "Action completed",
        detail: `${payload.actor} finished the action.`,
        tone: "neutral",
      };
    default:
      return {
        title: "Effect",
        detail: payload.type,
        tone: "neutral",
      };
  }
}

function formatPath(value: unknown): string {
  return Array.isArray(value) ? value.join(" > ") : "an action";
}

function flipCount(flips: unknown): number {
  return Array.isArray(flips) ? flips.length : 0;
}

function resourceCounts(value: unknown): string {
  if (!value || typeof value !== "object") {
    return "public resources";
  }
  const counts = value as { amber?: unknown; jade?: unknown; iron?: unknown };
  return `amber ${counts.amber ?? 0}, jade ${counts.jade ?? 0}, iron ${counts.iron ?? 0}`;
}

function siteLabel(value: unknown): string {
  if (typeof value !== "string") {
    return "the site";
  }
  return value.replace(/^site_/, "").replaceAll("_", " ");
}

function factionLabel(value: unknown): string {
  if (value === "faction_garrison") {
    return "Garrison";
  }
  if (value === "faction_prospectors") {
    return "Prospectors";
  }
  if (value === "faction_charter") {
    return "Charter";
  }
  if (value === "faction_freeholders") {
    return "Freeholders";
  }
  return typeof value === "string" ? value.replace(/^faction_/, "").replaceAll("_", " ") : "the faction";
}

function cardLabel(value: unknown): string {
  if (value && typeof value === "object" && "label" in value && typeof value.label === "string") {
    return value.label;
  }
  const text = String(value ?? "none");
  return text === "none" ? "none" : text.replace(/^ef_/, "").replaceAll("_", " ");
}

function formatSites(value: unknown): string {
  return Array.isArray(value) ? value.map(siteLabel).join(", ") : siteLabel(value);
}

function reckoningResult(value: unknown): string {
  if (value === "none" || value === null || typeof value === "undefined") {
    return "no instant victory; public scoring and income resolved";
  }
  return String(value);
}

function terminalOutcome(value: unknown, noun: string): string {
  if (!value || typeof value !== "object") {
    return `The ${noun} ended.`;
  }
  const outcome = value as { kind?: unknown; winner?: unknown };
  return outcome.kind === "draw" ? `The ${noun} ended in a draw.` : `${String(outcome.winner ?? "A seat")} won the ${noun}.`;
}

function riverStreetLabel(value: unknown): string {
  switch (value) {
    case "preflop":
      return "Preflop";
    case "flop":
      return "Flop";
    case "turn":
      return "Turn";
    case "river":
      return "River";
    default:
      return "Street";
  }
}

function riverPotLabel(value: unknown): string {
  if (value === "main_pot") {
    return "main pot";
  }
  if (typeof value === "string" && value.length > 0) {
    return "side pot";
  }
  return "pot";
}

function isPokerLiteOutcome(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const outcome = value as { kind?: unknown; shared_pool?: unknown };
  return (
    outcome.kind === "yield_win" ||
    outcome.kind === "showdown_win" ||
    outcome.kind === "split" ||
    typeof outcome.shared_pool === "number"
  );
}

function isPlainTricksOutcome(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const outcome = value as { kind?: unknown; totals?: unknown; each?: unknown };
  return (outcome.kind === "trick_win" || outcome.kind === "split") && typeof outcome.totals === "object";
}

function isMaskedClaimsOutcome(value: unknown): boolean {
  if (!value || typeof value !== "object") {
    return false;
  }
  const outcome = value as { kind?: unknown; scores?: unknown };
  return (
    outcome.kind === "score_win" ||
    outcome.kind === "tiebreak_win" ||
    outcome.kind === "draw" ||
    typeof outcome.scores === "object"
  );
}

export function summarizeEffect(entry: EffectEntry): string {
  const feedback = feedbackForEffect(entry);
  return `${entry.cursor}: ${feedback.title} - ${feedback.detail}`;
}

export function useReducedMotionPreference() {
  const [systemReduced, setSystemReduced] = useState(false);
  const [override, setOverrideState] = useState<ReducedMotionOverride>(() => readOverride());

  useEffect(() => {
    const query = window.matchMedia("(prefers-reduced-motion: reduce)");
    const update = () => setSystemReduced(query.matches);
    update();
    query.addEventListener("change", update);
    return () => query.removeEventListener("change", update);
  }, []);

  const setOverride = (next: ReducedMotionOverride) => {
    setOverrideState(next);
    if (next === "system") {
      window.localStorage.removeItem(STORAGE_KEY);
    } else {
      window.localStorage.setItem(STORAGE_KEY, next);
    }
  };

  return {
    reducedMotion: override === "system" ? systemReduced : override === "reduce",
    override,
    setOverride,
  };
}

function readOverride(): ReducedMotionOverride {
  if (typeof window === "undefined") {
    return "system";
  }
  const stored = window.localStorage.getItem(STORAGE_KEY);
  return stored === "reduce" || stored === "motion" ? stored : "system";
}

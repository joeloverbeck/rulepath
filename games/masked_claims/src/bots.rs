use ai_core::RandomLegalBot;
use engine_core::{ActionPath, ActionTree, Actor, Diagnostic, Seed, Viewer};

use crate::{
    actions::{legal_action_tree, parse_action_path, MaskedClaimsAction, ResponseChoice},
    ids::{
        Grade, MaskTileId, MaskedClaimsSeat, ACTION_CLAIM, ACTION_RESPOND_ACCEPT,
        ACTION_RESPOND_CHALLENGE, STANDARD_MASK_COUNT, STANDARD_TILES_PER_GRADE,
    },
    state::{MaskedClaimsState, Phase},
    visibility::{project_view, ExposedMaskView, PrivateView, PublicView},
};

pub const RANDOM_POLICY_ID: &str = "masked-claims-random-legal-v0";
pub const LEVEL1_POLICY_ID: &str = "masked-claims-level1-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskedClaimsBotInput {
    pub bot_seat: MaskedClaimsSeat,
    pub legal_action_tree: ActionTree,
    pub phase: Phase,
    pub turn_index: u8,
    pub claimant: MaskedClaimsSeat,
    pub pedestal_declared_grade: Option<Grade>,
    pub exposed_rows: [Vec<ExposedMaskView>; 2],
    pub scores: [u8; 2],
    pub own_hand: Vec<MaskTileId>,
}

impl MaskedClaimsBotInput {
    pub fn stable_summary(&self) -> String {
        format!(
            "seat={};choices={};phase={};turn={};claimant={};pedestal={};exposed={}|{};scores={},{};own_grades={}",
            self.bot_seat.as_str(),
            legal_actions_from_input(self).len(),
            self.phase.as_str(),
            self.turn_index,
            self.claimant.as_str(),
            self.pedestal_declared_grade
                .map(Grade::as_str)
                .unwrap_or("none"),
            exposed_summary(&self.exposed_rows[0]),
            exposed_summary(&self.exposed_rows[1]),
            self.scores[0],
            self.scores[1],
            self.own_hand
                .iter()
                .map(|tile| tile.grade().as_str())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub policy_id: String,
    pub policy_version: u32,
    pub level: u8,
    pub action_path: ActionPath,
    pub rationale: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaskedClaimsRandomBot {
    pub seed: Seed,
}

impl MaskedClaimsRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> MaskedClaimsBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let input = Self::input_for(state, bot_seat);
        RandomLegalBot::new(self.seed).select_action(&input.legal_action_tree)
    }

    pub fn select_decision(
        &self,
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random legal Masked Claims action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaskedClaimsLevel1Bot {
    pub seed: Seed,
}

impl MaskedClaimsLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn input_for(
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> MaskedClaimsBotInput {
        bot_input_for(state, bot_seat)
    }

    pub fn select_action(
        &self,
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }

    pub fn select_decision(
        &self,
        state: &MaskedClaimsState,
        bot_seat: MaskedClaimsSeat,
    ) -> Result<BotDecision, Diagnostic> {
        self.select_decision_from_input(&Self::input_for(state, bot_seat))
    }

    pub fn select_decision_from_input(
        &self,
        input: &MaskedClaimsBotInput,
    ) -> Result<BotDecision, Diagnostic> {
        let legal = legal_actions_from_input(input);
        if legal.is_empty() {
            return Err(no_legal_actions());
        }

        match input.phase {
            Phase::Claim { .. } => {
                let Some(claim) = choose_level1_claim(input, &legal, self.seed) else {
                    return Err(no_legal_actions());
                };
                Ok(decision(
                    1,
                    LEVEL1_POLICY_ID,
                    claim.action_path(),
                    claim_rationale(claim),
                ))
            }
            Phase::Reaction { .. } => {
                let response = choose_level1_response(input, &legal);
                Ok(decision(
                    1,
                    LEVEL1_POLICY_ID,
                    response.action_path(),
                    response_rationale(input, response),
                ))
            }
            Phase::Terminal => Err(no_legal_actions()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ClaimOption {
    tile: MaskTileId,
    declared: Grade,
}

impl ClaimOption {
    fn action_path(self) -> ActionPath {
        ActionPath {
            segments: vec![
                ACTION_CLAIM.to_owned(),
                self.tile.as_str().to_owned(),
                self.declared.as_str().to_owned(),
            ],
        }
    }
}

pub fn actor_for_seat(state: &MaskedClaimsState, seat: MaskedClaimsSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn action_from_decision(decision: &BotDecision) -> Option<MaskedClaimsAction> {
    parse_action_path(&decision.action_path.segments)
}

fn bot_input_for(state: &MaskedClaimsState, bot_seat: MaskedClaimsSeat) -> MaskedClaimsBotInput {
    bot_input_from_view(
        bot_seat,
        legal_action_tree(state, &actor_for_seat(state, bot_seat)),
        &project_view(
            state,
            &Viewer {
                seat_id: Some(state.seats[bot_seat.index()].clone()),
            },
        ),
    )
}

fn bot_input_from_view(
    bot_seat: MaskedClaimsSeat,
    legal_action_tree: ActionTree,
    view: &PublicView,
) -> MaskedClaimsBotInput {
    let own_hand = match &view.private_view {
        PrivateView::Seat(private) if private.seat == bot_seat => private
            .own_hand
            .iter()
            .filter_map(|mask| MaskTileId::parse(&mask.tile_id))
            .collect(),
        _ => Vec::new(),
    };

    MaskedClaimsBotInput {
        bot_seat,
        legal_action_tree,
        phase: view.phase,
        turn_index: view.turn_index,
        claimant: view.claimant,
        pedestal_declared_grade: view.pedestal.map(|claim| claim.declared_grade),
        exposed_rows: view.exposed_rows.clone(),
        scores: view.scores,
        own_hand,
    }
}

fn legal_actions_from_input(input: &MaskedClaimsBotInput) -> Vec<MaskedClaimsAction> {
    match input.phase {
        Phase::Claim { .. } => claim_options(input)
            .into_iter()
            .map(|claim| MaskedClaimsAction::Claim {
                tile: claim.tile,
                declared: claim.declared,
            })
            .collect(),
        Phase::Reaction { .. } => input
            .legal_action_tree
            .root
            .choices
            .iter()
            .filter_map(|choice| parse_action_path(std::slice::from_ref(&choice.segment)))
            .collect(),
        Phase::Terminal => Vec::new(),
    }
}

fn claim_options(input: &MaskedClaimsBotInput) -> Vec<ClaimOption> {
    input
        .legal_action_tree
        .root
        .choices
        .iter()
        .filter(|choice| choice.segment == ACTION_CLAIM)
        .filter_map(|choice| choice.next.as_ref())
        .flat_map(|node| node.choices.iter())
        .filter_map(|tile_choice| {
            let tile = MaskTileId::parse(&tile_choice.segment)?;
            Some((tile, tile_choice.next.as_ref()?))
        })
        .flat_map(|(tile, grade_node)| {
            grade_node.choices.iter().filter_map(move |grade_choice| {
                Some(ClaimOption {
                    tile,
                    declared: Grade::parse(&grade_choice.segment)?,
                })
            })
        })
        .collect()
}

fn choose_level1_claim(
    input: &MaskedClaimsBotInput,
    legal: &[MaskedClaimsAction],
    seed: Seed,
) -> Option<ClaimOption> {
    let claims = legal.iter().filter_map(|action| match action {
        MaskedClaimsAction::Claim { tile, declared } => Some(ClaimOption {
            tile: *tile,
            declared: *declared,
        }),
        MaskedClaimsAction::Response(_) => None,
    });
    let posture = seed.0 % 6;
    let preferred_kind = if posture <= 1 {
        ClaimKind::Bluff
    } else if posture == 2 {
        ClaimKind::Underclaim
    } else {
        ClaimKind::Honest
    };

    claims
        .clone()
        .filter(|claim| claim_kind(*claim) == preferred_kind)
        .filter(|claim| !is_certain_lie_claim(input, *claim))
        .max_by_key(|claim| claim_rank(*claim, seed))
        .or_else(|| {
            claims
                .filter(|claim| claim_kind(*claim) == ClaimKind::Honest)
                .max_by_key(|claim| claim_rank(*claim, seed))
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ClaimKind {
    Honest,
    Underclaim,
    Bluff,
}

fn claim_kind(claim: ClaimOption) -> ClaimKind {
    match claim.declared.value().cmp(&claim.tile.grade().value()) {
        std::cmp::Ordering::Equal => ClaimKind::Honest,
        std::cmp::Ordering::Less => ClaimKind::Underclaim,
        std::cmp::Ordering::Greater => ClaimKind::Bluff,
    }
}

fn claim_rank(claim: ClaimOption, seed: Seed) -> (u8, u8, std::cmp::Reverse<&'static str>, u64) {
    (
        claim.declared.value(),
        claim.tile.grade().value(),
        std::cmp::Reverse(claim.tile.as_str()),
        seeded_tie(seed, claim.tile.as_str(), claim.declared.as_str()),
    )
}

fn is_certain_lie_claim(input: &MaskedClaimsBotInput, claim: ClaimOption) -> bool {
    claim.declared != claim.tile.grade()
        && known_grade_count(input, claim.declared) >= STANDARD_TILES_PER_GRADE
}

fn choose_level1_response(
    input: &MaskedClaimsBotInput,
    legal: &[MaskedClaimsAction],
) -> ResponseChoice {
    let can_challenge = legal.contains(&MaskedClaimsAction::Response(ResponseChoice::Challenge));
    let can_accept = legal.contains(&MaskedClaimsAction::Response(ResponseChoice::Accept));
    let Some(declared) = input.pedestal_declared_grade else {
        return if can_accept {
            ResponseChoice::Accept
        } else {
            ResponseChoice::Challenge
        };
    };
    if can_challenge && certain_lie_by_count(input, declared) {
        return ResponseChoice::Challenge;
    }
    if can_challenge && threshold_challenge(input, declared) {
        return ResponseChoice::Challenge;
    }
    if can_accept {
        ResponseChoice::Accept
    } else {
        ResponseChoice::Challenge
    }
}

fn certain_lie_by_count(input: &MaskedClaimsBotInput, declared: Grade) -> bool {
    known_grade_count(input, declared) >= STANDARD_TILES_PER_GRADE
}

fn threshold_challenge(input: &MaskedClaimsBotInput, declared: Grade) -> bool {
    let known = known_grade_count(input, declared);
    let remaining_grade_copies = STANDARD_TILES_PER_GRADE.saturating_sub(known);
    let unseen_tiles = STANDARD_MASK_COUNT
        .saturating_sub(public_exposed_count(input))
        .saturating_sub(input.own_hand.len() as u8)
        .max(1);
    let threshold_percent = 20 + declared.value() * 5;

    u16::from(remaining_grade_copies) * 100 < u16::from(unseen_tiles) * u16::from(threshold_percent)
}

fn known_grade_count(input: &MaskedClaimsBotInput, grade: Grade) -> u8 {
    let own = input
        .own_hand
        .iter()
        .filter(|tile| tile.grade() == grade)
        .count() as u8;
    own + input
        .exposed_rows
        .iter()
        .flatten()
        .filter(|mask| mask.actual_grade == grade)
        .count() as u8
}

fn public_exposed_count(input: &MaskedClaimsBotInput) -> u8 {
    input.exposed_rows.iter().map(Vec::len).sum::<usize>() as u8
}

fn claim_rationale(claim: ClaimOption) -> String {
    match claim_kind(claim) {
        ClaimKind::Honest => format!(
            "Claimed {} from a held {} using the highest safe legal claim.",
            claim.declared.label(),
            claim.tile.grade().label()
        ),
        ClaimKind::Underclaim => format!(
            "Claimed {} below a held {} to keep the claim plausible.",
            claim.declared.label(),
            claim.tile.grade().label()
        ),
        ClaimKind::Bluff => format!(
            "Claimed {} above a held {} using the bounded bluff policy.",
            claim.declared.label(),
            claim.tile.grade().label()
        ),
    }
}

fn response_rationale(input: &MaskedClaimsBotInput, response: ResponseChoice) -> String {
    let Some(declared) = input.pedestal_declared_grade else {
        return format!("{}: selected a legal response.", response.label());
    };
    match response {
        ResponseChoice::Challenge if certain_lie_by_count(input, declared) => format!(
            "Challenged: all {} {} masks are already visible to me.",
            STANDARD_TILES_PER_GRADE,
            declared.label()
        ),
        ResponseChoice::Challenge => format!(
            "Challenged: the {} claim is below the public-counting threshold.",
            declared.label()
        ),
        ResponseChoice::Accept => format!(
            "Accepted: a {} claim remains plausible and is worth {}.",
            declared.label(),
            declared.value()
        ),
    }
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    BotDecision {
        policy_id: policy_id.to_owned(),
        policy_version: 1,
        level,
        action_path,
        rationale,
    }
}

impl ResponseChoice {
    fn action_path(self) -> ActionPath {
        ActionPath {
            segments: vec![match self {
                Self::Accept => ACTION_RESPOND_ACCEPT.to_owned(),
                Self::Challenge => ACTION_RESPOND_CHALLENGE.to_owned(),
            }],
        }
    }
}

fn no_legal_actions() -> Diagnostic {
    Diagnostic {
        code: "no_legal_actions".to_owned(),
        message: "no legal action is available".to_owned(),
    }
}

fn seeded_tie(seed: Seed, left: &str, right: &str) -> u64 {
    let mut value = seed.0;
    for byte in left.bytes().chain(right.bytes()) {
        value ^= u64::from(byte);
        value = value.wrapping_mul(0x100_0000_01b3);
    }
    value
}

fn exposed_summary(row: &[ExposedMaskView]) -> String {
    if row.is_empty() {
        return "none".to_owned();
    }
    row.iter()
        .map(|mask| {
            format!(
                "{}:{}:{}",
                mask.actual_grade.as_str(),
                mask.declared_grade.as_str(),
                mask.challenger.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::{validate_command, ValidatedAction},
        apply_action,
        setup::{setup_match, SetupOptions},
    };
    use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId};

    fn standard_state(seed: u64) -> MaskedClaimsState {
        setup_match(
            Seed(seed),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn command(
        state: &MaskedClaimsState,
        seat: MaskedClaimsSeat,
        action_path: ActionPath,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor_for_seat(state, seat),
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn apply_first_claim(state: &mut MaskedClaimsState) {
        let tile = state.hand_for_internal(MaskedClaimsSeat::Seat0)[0];
        let action_path = ActionPath {
            segments: vec![
                ACTION_CLAIM.to_owned(),
                tile.as_str().to_owned(),
                Grade::Master.as_str().to_owned(),
            ],
        };
        let ValidatedAction::Claim(claim) =
            validate_command(state, &command(state, MaskedClaimsSeat::Seat0, action_path))
                .expect("claim validates")
        else {
            panic!("expected claim");
        };
        apply_action(state, ValidatedAction::Claim(claim)).expect("claim applies");
    }

    fn assert_decision_validates(
        state: &MaskedClaimsState,
        seat: MaskedClaimsSeat,
        decision: &BotDecision,
    ) {
        validate_command(state, &command(state, seat, decision.action_path.clone()))
            .expect("bot decision validates");
    }

    #[test]
    fn random_bot_selects_legal_actions_in_claim_and_response_phases() {
        let mut state = standard_state(13);
        let claim = MaskedClaimsRandomBot::new(Seed(3))
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("claim decision");
        assert_decision_validates(&state, MaskedClaimsSeat::Seat0, &claim);

        apply_first_claim(&mut state);
        let response = MaskedClaimsRandomBot::new(Seed(4))
            .select_decision(&state, MaskedClaimsSeat::Seat1)
            .expect("response decision");
        assert_decision_validates(&state, MaskedClaimsSeat::Seat1, &response);
    }

    #[test]
    fn level1_bot_selects_legal_actions_in_claim_and_response_phases() {
        let mut state = standard_state(14);
        let claim = MaskedClaimsLevel1Bot::new(Seed(10))
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("claim decision");
        assert_decision_validates(&state, MaskedClaimsSeat::Seat0, &claim);

        apply_first_claim(&mut state);
        let response = MaskedClaimsLevel1Bot::new(Seed(10))
            .select_decision(&state, MaskedClaimsSeat::Seat1)
            .expect("response decision");
        assert_decision_validates(&state, MaskedClaimsSeat::Seat1, &response);
    }

    #[test]
    fn level1_is_deterministic_for_same_allowed_view_and_seed() {
        let state = standard_state(15);
        let bot = MaskedClaimsLevel1Bot::new(Seed(42));

        let first = bot
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("first decision");
        let second = bot
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("second decision");

        assert_eq!(first, second);
    }

    #[test]
    fn level1_response_ignores_hidden_pedestal_identity_when_view_is_same() {
        let mut state = standard_state(16);
        apply_first_claim(&mut state);
        let input = MaskedClaimsLevel1Bot::input_for(&state, MaskedClaimsSeat::Seat1);
        let mut alternate = input.clone();
        alternate.legal_action_tree = input.legal_action_tree.clone();

        let bot = MaskedClaimsLevel1Bot::new(Seed(51));
        let first = bot
            .select_decision_from_input(&input)
            .expect("first decision");
        let second = bot
            .select_decision_from_input(&alternate)
            .expect("second decision");

        assert_eq!(input.stable_summary(), alternate.stable_summary());
        assert_eq!(first, second);
        assert_no_hidden_terms(&first.rationale);
    }

    fn assert_no_hidden_terms(rationale: &str) {
        assert!(!rationale.contains("mask_g"));
        assert!(!rationale.contains("reserve"));
        assert!(!rationale.contains("opponent hand"));
        assert!(!rationale.contains("pedestal tile"));
    }
}

use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, CommandEnvelope,
    Diagnostic,
};

use crate::{
    ids::{Grade, MaskTileId, MaskedClaimsSeat, ACTION_CLAIM},
    state::{MaskedClaimsState, Phase},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MaskedClaimsAction {
    pub tile: MaskTileId,
    pub declared: Grade,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedClaim {
    pub actor: MaskedClaimsSeat,
    pub tile: MaskTileId,
    pub declared: Grade,
    pub turn_index: u8,
}

pub fn legal_action_tree(state: &MaskedClaimsState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor_seat)
        || state.claimant != actor_seat
        || !matches!(state.phase, Phase::Claim { .. })
    {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let tiles = claimable_tiles(state, actor_seat);
    if tiles.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut claim_choice = ActionChoice::leaf(ACTION_CLAIM, "Claim", "Claim a mask grade");
    claim_choice.metadata = public_claim_metadata(state, actor_seat);
    claim_choice.tags = vec!["claim".to_owned(), "mask-choice".to_owned()];
    claim_choice.preview = ActionPreview::Available;
    claim_choice.next = Some(Box::new(ActionNode {
        choices: tiles
            .into_iter()
            .map(|tile| tile_choice(state, actor_seat, tile))
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![claim_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn claimable_tiles(state: &MaskedClaimsState, actor: MaskedClaimsSeat) -> Vec<MaskTileId> {
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor)
        || state.claimant != actor
        || !matches!(state.phase, Phase::Claim { .. })
    {
        return Vec::new();
    }

    state.hand_for_internal(actor).to_vec()
}

pub fn actor_seat(state: &MaskedClaimsState, actor: &Actor) -> Option<MaskedClaimsSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(MaskedClaimsSeat::from_index)
}

pub fn parse_action_path(segments: &[String]) -> Option<MaskedClaimsAction> {
    let [family, tile, declared] = segments else {
        return None;
    };
    if family != ACTION_CLAIM {
        return None;
    }
    let tile = MaskTileId::parse(tile)?;
    let declared = parse_grade_segment(declared)?;
    Some(MaskedClaimsAction { tile, declared })
}

pub fn validate_command(
    state: &MaskedClaimsState,
    command: &CommandEnvelope,
) -> Result<ValidatedClaim, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(terminal_diagnostic());
    }
    if !matches!(state.phase, Phase::Claim { .. }) {
        return Err(wrong_phase_diagnostic());
    }
    if state.active_seat != Some(actor) || state.claimant != actor {
        return Err(wrong_claimant_diagnostic());
    }

    validate_claim_segments(&command.action_path.segments)?;
    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;

    if !state.hand_for_internal(actor).contains(&action.tile) {
        return Err(mask_not_in_hand_diagnostic());
    }

    if !claimable_tiles(state, actor).contains(&action.tile) {
        return Err(unavailable_action_diagnostic());
    }

    Ok(ValidatedClaim {
        actor,
        tile: action.tile,
        declared: action.declared,
        turn_index: state.turn_index,
    })
}

pub fn command_public_summary(validated: ValidatedClaim) -> String {
    format!("claim/grade-{}", validated.declared.as_str())
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "only a seated Masked Claims actor may submit a claim".to_owned(),
    }
}

pub fn wrong_claimant_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_claimant".to_owned(),
        message: "only the current claimant may choose a mask claim".to_owned(),
    }
}

pub fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: "a mask claim can only be submitted during the claim phase".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "claims cannot be submitted after the match is complete".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "Masked Claims claim actions require claim, mask, and declared grade segments"
            .to_owned(),
    }
}

pub fn invalid_grade_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "invalid_grade".to_owned(),
        message: "the declared grade must be between 1 and 5".to_owned(),
    }
}

pub fn mask_not_in_hand_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "mask_not_in_hand".to_owned(),
        message: "the submitted mask is not in the actor's hand".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "the requested mask claim is not available at this decision point".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

fn validate_claim_segments(segments: &[String]) -> Result<(), Diagnostic> {
    let [family, _tile, declared] = segments else {
        return Err(malformed_action_diagnostic());
    };
    if family != ACTION_CLAIM {
        return Err(malformed_action_diagnostic());
    }
    if parse_grade_segment(declared).is_none() {
        return Err(invalid_grade_diagnostic());
    }
    Ok(())
}

fn parse_grade_segment(value: &str) -> Option<Grade> {
    Grade::ALL.into_iter().find(|grade| grade.as_str() == value)
}

fn tile_choice(
    state: &MaskedClaimsState,
    actor: MaskedClaimsSeat,
    tile: MaskTileId,
) -> ActionChoice {
    let label = format!("Mask {}", tile.label());
    let accessibility_label = format!("Claim with {label}");
    let mut choice = ActionChoice::leaf(tile.as_str(), label, accessibility_label.clone());
    choice.metadata = vec![
        metadata("action_family", ACTION_CLAIM),
        metadata("turn_index", state.turn_index.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("own_mask_grade", tile.grade().as_str()),
        metadata("own_mask_label", tile.grade().label()),
        metadata("accessibility_copy", accessibility_label),
    ];
    choice.tags = vec![
        "claim".to_owned(),
        "mask".to_owned(),
        format!("grade-{}", tile.grade().as_str()),
    ];
    choice.preview = ActionPreview::Available;
    choice.next = Some(Box::new(ActionNode {
        choices: Grade::ALL
            .into_iter()
            .map(|grade| declared_grade_choice(state, actor, grade))
            .collect(),
    }));
    choice
}

fn declared_grade_choice(
    state: &MaskedClaimsState,
    actor: MaskedClaimsSeat,
    grade: Grade,
) -> ActionChoice {
    let label = format!("Declare {}", grade.label());
    let accessibility_label = format!("Declare {} for the claim", grade.label());
    let mut choice = ActionChoice::leaf(grade.as_str(), label, accessibility_label.clone());
    choice.metadata = vec![
        metadata("action_family", ACTION_CLAIM),
        metadata("turn_index", state.turn_index.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("declared_grade", grade.as_str()),
        metadata("declared_label", grade.label()),
        metadata("public_score_preview", grade.value().to_string()),
        metadata("public_summary", format!("claim/grade-{}", grade.as_str())),
        metadata("accessibility_copy", accessibility_label),
    ];
    choice.tags = vec![
        "claim".to_owned(),
        "declared-grade".to_owned(),
        format!("grade-{}", grade.as_str()),
    ];
    choice.preview = ActionPreview::Available;
    choice
}

fn public_claim_metadata(
    state: &MaskedClaimsState,
    actor: MaskedClaimsSeat,
) -> Vec<ActionMetadata> {
    vec![
        metadata("action_family", ACTION_CLAIM),
        metadata("turn_index", state.turn_index.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("claimant", state.claimant.as_str()),
        metadata("phase", "claim"),
    ]
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::{setup_match, SetupOptions};
    use engine_core::{ActionPath, FreshnessToken, RulesVersion, SeatId, Seed};

    fn actor(seat: &str) -> Actor {
        Actor {
            seat_id: SeatId(seat.to_owned()),
        }
    }

    fn command(state: &MaskedClaimsState, seat: &str, segments: Vec<&str>) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn standard_state() -> MaskedClaimsState {
        setup_match(
            Seed(1),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn claim_leaf_paths(tree: &ActionTree) -> Vec<Vec<String>> {
        let Some(claim) = tree.root.choices.first() else {
            return Vec::new();
        };
        let Some(tile_node) = claim.next.as_ref() else {
            return Vec::new();
        };

        tile_node
            .choices
            .iter()
            .flat_map(|tile| {
                tile.next.as_ref().into_iter().flat_map(move |grade_node| {
                    grade_node.choices.iter().map(move |grade| {
                        vec![
                            claim.segment.clone(),
                            tile.segment.clone(),
                            grade.segment.clone(),
                        ]
                    })
                })
            })
            .collect()
    }

    #[test]
    fn claimant_tree_contains_each_held_mask_by_each_grade() {
        let state = standard_state();
        let tree = legal_action_tree(&state, &actor("seat_0"));
        let paths = claim_leaf_paths(&tree);

        assert_eq!(paths.len(), 25);
        for tile in state.hand_for_internal(MaskedClaimsSeat::Seat0) {
            for grade in Grade::ALL {
                assert!(paths.contains(&vec![
                    ACTION_CLAIM.to_owned(),
                    tile.as_str().to_owned(),
                    grade.as_str().to_owned()
                ]));
            }
        }
    }

    #[test]
    fn responder_and_unseated_actor_receive_empty_tree() {
        let state = standard_state();

        assert!(legal_action_tree(&state, &actor("seat_1"))
            .root
            .choices
            .is_empty());
        assert!(legal_action_tree(&state, &actor("seat_x"))
            .root
            .choices
            .is_empty());
    }

    #[test]
    fn claim_metadata_does_not_copy_tile_id_to_public_fields() {
        let state = standard_state();
        let tree = legal_action_tree(&state, &actor("seat_0"));
        let rendered = format!("{:?}", tree.root.choices[0].metadata);
        assert_no_tile_ids(&rendered);

        let tile_choice = &tree.root.choices[0]
            .next
            .as_ref()
            .expect("tile node")
            .choices[0];
        let tile_metadata = format!("{:?}", tile_choice.metadata);
        assert_no_tile_ids(&tile_metadata);
        assert!(tile_metadata.contains("own_mask_grade"));

        let grade_choice = &tile_choice.next.as_ref().expect("grade node").choices[0];
        let grade_metadata = format!("{:?}", grade_choice.metadata);
        assert_no_tile_ids(&grade_metadata);
        assert!(grade_metadata.contains("claim/grade-1"));
    }

    #[test]
    fn validate_accepts_legal_claim_path_and_redacts_summary() {
        let state = standard_state();
        let tile = state.hand_for_internal(MaskedClaimsSeat::Seat0)[0];

        let validated = validate_command(
            &state,
            &command(&state, "seat_0", vec!["claim", tile.as_str(), "5"]),
        )
        .expect("claim validates");

        assert_eq!(validated.actor, MaskedClaimsSeat::Seat0);
        assert_eq!(validated.tile, tile);
        assert_eq!(validated.declared, Grade::Master);
        assert_eq!(validated.turn_index, 0);
        assert_eq!(command_public_summary(validated), "claim/grade-5");
        assert!(!command_public_summary(validated).contains(tile.as_str()));
    }

    #[test]
    fn validation_rejects_stale_wrong_actor_terminal_and_malformed() {
        let mut state = standard_state();
        let tile = state.hand_for_internal(MaskedClaimsSeat::Seat0)[0];

        let mut stale = command(&state, "seat_0", vec!["claim", tile.as_str(), "1"]);
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).expect_err("stale").code,
            "stale_action"
        );

        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_x", vec!["claim", tile.as_str(), "1"])
            )
            .expect_err("unseated")
            .code,
            "wrong_seat"
        );

        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_1", vec!["claim", tile.as_str(), "1"])
            )
            .expect_err("wrong claimant")
            .code,
            "wrong_claimant"
        );

        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["claim", tile.as_str()])
            )
            .expect_err("malformed")
            .code,
            "malformed_action"
        );

        state.phase = Phase::Terminal;
        state.terminal_outcome = Some(crate::TerminalOutcome::Draw { scores: [0, 0] });
        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["claim", tile.as_str(), "1"])
            )
            .expect_err("terminal")
            .code,
            "terminal_state"
        );
    }

    #[test]
    fn validation_rejects_wrong_phase_unowned_tile_and_bad_grade_without_leaking_hand() {
        let mut state = standard_state();
        let own_tile = state.hand_for_internal(MaskedClaimsSeat::Seat0)[0];
        let opponent_tile = state.hand_for_internal(MaskedClaimsSeat::Seat1)[0];

        state.phase = Phase::Reaction {
            turn_index: 0,
            responder: MaskedClaimsSeat::Seat1,
        };
        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["claim", own_tile.as_str(), "1"])
            )
            .expect_err("wrong phase")
            .code,
            "wrong_phase"
        );

        state.phase = Phase::Claim { turn_index: 0 };
        let not_in_hand = validate_command(
            &state,
            &command(&state, "seat_0", vec!["claim", opponent_tile.as_str(), "1"]),
        )
        .expect_err("not in hand");
        assert_eq!(not_in_hand.code, "mask_not_in_hand");
        assert!(!not_in_hand.message.contains(opponent_tile.as_str()));
        assert!(!not_in_hand.message.contains(own_tile.as_str()));

        let invalid_grade = validate_command(
            &state,
            &command(&state, "seat_0", vec!["claim", own_tile.as_str(), "6"]),
        )
        .expect_err("invalid grade");
        assert_eq!(invalid_grade.code, "invalid_grade");
        assert!(!invalid_grade.message.contains(own_tile.as_str()));
    }

    fn assert_no_tile_ids(value: &str) {
        for tile in MaskTileId::ALL {
            assert!(
                !value.contains(tile.as_str()),
                "unexpected tile id {} in {value}",
                tile.as_str()
            );
        }
    }
}

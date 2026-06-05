//! Browser-facing Rulepath API surface.

use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    slice, str,
};

use engine_core::{
    ActionChoice, ActionPath, ActionTree, Actor, CommandEnvelope, EffectCursor, EffectEnvelope,
    EffectLog, RulesVersion, SeatId, Seed, Viewer, VisibilityScope,
};
use race_to_n::{
    apply_action as race_apply_action, legal_action_tree, project_view, setup_match, RaceEffect,
    RaceRandomBot, RaceSeat, RaceState, SetupOptions,
};

const PLACEHOLDER_VERSION: &str = "rulepath-wasm-api/0.1.0";
const GAME_RACE_TO_N: &str = "race_to_n";
const RULES_VERSION: u32 = 1;

thread_local! {
    static MATCHES: RefCell<BTreeMap<String, MatchRecord>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT_MATCH_ID: Cell<u64> = const { Cell::new(1) };
    static LAST_OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
}

#[derive(Clone, Debug)]
struct MatchRecord {
    game_id: String,
    state: RaceState,
    effects: EffectLog<RaceEffect>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegisteredGame {
    RaceToN,
}

pub fn placeholder_version() -> &'static str {
    PLACEHOLDER_VERSION
}

pub fn new_match(game_id: &str, seed: u64) -> Result<String, String> {
    match resolve_game(game_id)? {
        RegisteredGame::RaceToN => {
            let seats = seats();
            let state = setup_match(Seed(seed), &seats, &SetupOptions::default())
                .map_err(diagnostic_json)?;
            let match_id = next_match_id(game_id);
            MATCHES.with(|matches| {
                matches.borrow_mut().insert(
                    match_id.clone(),
                    MatchRecord {
                        game_id: game_id.to_owned(),
                        state,
                        effects: EffectLog::new(),
                    },
                );
            });
            Ok(format!(
                "{{\"match_id\":\"{}\",\"game_id\":\"{}\"}}",
                escape_json(&match_id),
                escape_json(game_id)
            ))
        }
    }
}

pub fn get_view(match_id: &str, _viewer_seat: Option<&str>) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        Ok(project_view(&record.state).to_json())
    })
}

pub fn get_action_tree(match_id: &str, actor_seat: &str) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        let actor = actor_for_seat(&record.state, parse_seat(actor_seat)?)?;
        Ok(action_tree_json(&legal_action_tree(&record.state, &actor)))
    })
}

pub fn apply_action(
    match_id: &str,
    actor_seat: &str,
    action_path: &str,
    freshness_token: u64,
) -> Result<String, String> {
    with_match_mut(match_id, |record| {
        resolve_game(&record.game_id)?;
        let seat = parse_seat(actor_seat)?;
        let command = CommandEnvelope {
            actor: actor_for_seat(&record.state, seat)?,
            action_path: parse_action_path(action_path),
            freshness_token: engine_core::FreshnessToken(freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            race_to_n::validate_command(&record.state, &command).map_err(diagnostic_json)?;
        let effects = race_apply_action(&mut record.state, action);
        let effect_json = effects_json(&effects);
        for effect in effects {
            record.effects.push(effect);
        }
        Ok(format!(
            "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
            effect_json,
            project_view(&record.state).to_json()
        ))
    })
}

pub fn run_bot_turn(match_id: &str, actor_seat: &str, bot_seed: u64) -> Result<String, String> {
    with_match_mut(match_id, |record| {
        resolve_game(&record.game_id)?;
        let seat = parse_seat(actor_seat)?;
        let bot = RaceRandomBot::new(Seed(bot_seed));
        let action_path = bot
            .select_action(&record.state, seat)
            .map_err(diagnostic_json)?;
        let command = CommandEnvelope {
            actor: actor_for_seat(&record.state, seat)?,
            action_path,
            freshness_token: record.state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            race_to_n::validate_command(&record.state, &command).map_err(diagnostic_json)?;
        let effects = race_apply_action(&mut record.state, action);
        let effect_json = effects_json(&effects);
        for effect in effects {
            record.effects.push(effect);
        }
        Ok(format!(
            "{{\"ok\":true,\"effects\":{},\"view\":{}}}",
            effect_json,
            project_view(&record.state).to_json()
        ))
    })
}

pub fn get_effects(
    match_id: &str,
    since_cursor: u64,
    viewer_seat: Option<&str>,
) -> Result<String, String> {
    with_match(match_id, |record| {
        resolve_game(&record.game_id)?;
        let viewer = viewer_for_seat(&record.state, viewer_seat)?;
        let effects = record
            .effects
            .since(EffectCursor(since_cursor), &viewer)
            .into_iter()
            .map(|logged| {
                format!(
                    "{{\"cursor\":{},\"effect\":{}}}",
                    logged.cursor.0,
                    effect_json(&logged.envelope)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Ok(format!("[{effects}]"))
    })
}

fn resolve_game(game_id: &str) -> Result<RegisteredGame, String> {
    match game_id {
        GAME_RACE_TO_N => Ok(RegisteredGame::RaceToN),
        _ => Err(format!(
            "{{\"code\":\"unknown_game\",\"message\":\"unsupported game id: {}\"}}",
            escape_json(game_id)
        )),
    }
}

fn next_match_id(game_id: &str) -> String {
    NEXT_MATCH_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-{id}")
    })
}

fn with_match<T>(
    match_id: &str,
    read: impl FnOnce(&MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let matches = matches.borrow();
        let record = matches
            .get(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        read(record)
    })
}

fn with_match_mut<T>(
    match_id: &str,
    update: impl FnOnce(&mut MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let mut matches = matches.borrow_mut();
        let record = matches
            .get_mut(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        update(record)
    })
}

fn missing_match_json(match_id: &str) -> String {
    format!(
        "{{\"code\":\"unknown_match\",\"message\":\"unknown match id: {}\"}}",
        escape_json(match_id)
    )
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn parse_seat(value: &str) -> Result<RaceSeat, String> {
    RaceSeat::parse(value).ok_or_else(|| {
        format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )
    })
}

fn actor_for_seat(state: &RaceState, seat: RaceSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

fn viewer_for_seat(state: &RaceState, seat: Option<&str>) -> Result<Viewer, String> {
    let seat_id = seat
        .map(parse_seat)
        .transpose()?
        .map(|seat| state.seats[seat.index()].clone());
    Ok(Viewer { seat_id })
}

fn parse_action_path(action_path: &str) -> ActionPath {
    ActionPath {
        segments: action_path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(str::to_owned)
            .collect(),
    }
}

fn action_tree_json(tree: &ActionTree) -> String {
    let choices = tree
        .root
        .choices
        .iter()
        .map(action_choice_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"freshness_token\":{},\"choices\":[{}]}}",
        tree.freshness_token.0, choices
    )
}

fn action_choice_json(choice: &ActionChoice) -> String {
    let metadata = choice
        .metadata
        .iter()
        .map(|entry| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                escape_json(&entry.key),
                escape_json(&entry.value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let tags = choice
        .tags
        .iter()
        .map(|tag| format!("\"{}\"", escape_json(tag)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"tags\":[{}]}}",
        escape_json(&choice.segment),
        escape_json(&choice.label),
        escape_json(&choice.accessibility_label),
        metadata,
        tags
    )
}

fn effects_json(effects: &[EffectEnvelope<RaceEffect>]) -> String {
    let body = effects
        .iter()
        .map(effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn effect_json(effect: &EffectEnvelope<RaceEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "\"public\"".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => {
            format!("{{\"private_to_seat\":\"{}\"}}", escape_json(&seat.0))
        }
    };
    let payload = match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => format!(
            "{{\"type\":\"action_started\",\"actor\":\"{}\",\"amount\":{}}}",
            actor.as_str(),
            amount
        ),
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "{{\"type\":\"counter_advanced\",\"actor\":\"{}\",\"from\":{},\"to\":{},\"amount\":{}}}",
            actor.as_str(),
            from.0,
            to.0,
            amount
        ),
        RaceEffect::TurnChanged { next_actor } => format!(
            "{{\"type\":\"turn_changed\",\"next_actor\":\"{}\"}}",
            next_actor.as_str()
        ),
        RaceEffect::GameEnded { winner } => {
            format!("{{\"type\":\"game_ended\",\"winner\":\"{}\"}}", winner.as_str())
        }
        RaceEffect::ActionCompleted { actor } => format!(
            "{{\"type\":\"action_completed\",\"actor\":\"{}\"}}",
            actor.as_str()
        ),
    };
    format!("{{\"visibility\":{},\"payload\":{}}}", visibility, payload)
}

fn diagnostic_json(diagnostic: engine_core::Diagnostic) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(&diagnostic.code),
        escape_json(&diagnostic.message)
    )
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_ptr() -> *const u8 {
    PLACEHOLDER_VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn rulepath_placeholder_version_len() -> usize {
    PLACEHOLDER_VERSION.len()
}

#[no_mangle]
pub extern "C" fn rulepath_alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

#[no_mangle]
/// # Safety
///
/// `ptr` must have been returned by `rulepath_alloc` with the same `len`, and it
/// must not be used after this call.
pub unsafe extern "C" fn rulepath_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        drop(unsafe { Vec::from_raw_parts(ptr, 0, len) });
    }
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_ptr() -> *const u8 {
    LAST_OUTPUT.with(|output| output.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn rulepath_last_output_len() -> usize {
    LAST_OUTPUT.with(|output| output.borrow().len())
}

#[no_mangle]
/// # Safety
///
/// `game_ptr..game_ptr + game_len` must be a valid UTF-8 buffer for the duration
/// of the call.
pub unsafe extern "C" fn rulepath_new_match(
    game_ptr: *const u8,
    game_len: usize,
    seed: u64,
) -> i32 {
    let game_id = unsafe { read_string(game_ptr, game_len) };
    write_result(game_id.and_then(|game_id| new_match(&game_id, seed)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be a valid UTF-8 buffer for the
/// duration of the call.
pub unsafe extern "C" fn rulepath_get_view(match_ptr: *const u8, match_len: usize) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    write_result(match_id.and_then(|match_id| get_view(&match_id, None)))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_get_action_tree(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id.and_then(|match_id| seat.and_then(|seat| get_action_tree(&match_id, &seat))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr`, `seat_ptr`, and `path_ptr` with their lengths must be valid
/// UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_apply_action(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    path_ptr: *const u8,
    path_len: usize,
    freshness_token: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    let path = unsafe { read_string(path_ptr, path_len) };
    write_result(match_id.and_then(|match_id| {
        seat.and_then(|seat| {
            path.and_then(|path| apply_action(&match_id, &seat, &path, freshness_token))
        })
    }))
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` and `seat_ptr..seat_ptr + seat_len` must
/// be valid UTF-8 buffers for the duration of the call.
pub unsafe extern "C" fn rulepath_run_bot_turn(
    match_ptr: *const u8,
    match_len: usize,
    seat_ptr: *const u8,
    seat_len: usize,
    bot_seed: u64,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let seat = unsafe { read_string(seat_ptr, seat_len) };
    write_result(
        match_id
            .and_then(|match_id| seat.and_then(|seat| run_bot_turn(&match_id, &seat, bot_seed))),
    )
}

#[no_mangle]
/// # Safety
///
/// `match_ptr..match_ptr + match_len` must be valid UTF-8. If `viewer_len` is
/// nonzero, `viewer_ptr..viewer_ptr + viewer_len` must also be valid UTF-8.
pub unsafe extern "C" fn rulepath_get_effects(
    match_ptr: *const u8,
    match_len: usize,
    since_cursor: u64,
    viewer_ptr: *const u8,
    viewer_len: usize,
) -> i32 {
    let match_id = unsafe { read_string(match_ptr, match_len) };
    let viewer = if viewer_len == 0 {
        Ok(None)
    } else {
        unsafe { read_string(viewer_ptr, viewer_len) }.map(Some)
    };
    write_result(match_id.and_then(|match_id| {
        viewer.and_then(|viewer| get_effects(&match_id, since_cursor, viewer.as_deref()))
    }))
}

unsafe fn read_string(ptr: *const u8, len: usize) -> Result<String, String> {
    if ptr.is_null() && len > 0 {
        return Err(
            "{\"code\":\"invalid_pointer\",\"message\":\"input pointer is null\"}".to_owned(),
        );
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len) };
    str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| "{\"code\":\"invalid_utf8\",\"message\":\"input is not utf-8\"}".to_owned())
}

fn write_result(result: Result<String, String>) -> i32 {
    match result {
        Ok(output) => {
            write_output(output);
            0
        }
        Err(error) => {
            write_output(error);
            1
        }
    }
}

fn write_output(output: String) {
    LAST_OUTPUT.with(|last| {
        *last.borrow_mut() = output;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_version_is_stable() {
        assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
    }

    #[test]
    fn surface_drives_minimal_turn_loop() {
        let created = new_match("race_to_n", 11).expect("match created");
        let match_id = extract_match_id(&created);

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":0"));
        assert!(!view.contains("seat-0"));

        let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
        assert!(tree.contains("\"segment\":\"add-1\""));
        assert!(tree.contains("\"freshness_token\":0"));

        let applied = apply_action(&match_id, "seat_0", "add-1", 0).expect("human action applies");
        assert!(applied.contains("\"counter\":1"));
        assert!(applied.contains("\"type\":\"counter_advanced\""));

        let effects = get_effects(&match_id, 0, None).expect("effects returned");
        assert!(effects.contains("\"cursor\":1"));
        assert!(effects.contains("\"visibility\":\"public\""));

        let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
        assert!(bot.contains("\"ok\":true"));
        assert!(bot.contains("\"active_seat\":\"seat_0\""));
    }

    #[test]
    fn stale_action_returns_diagnostic_without_mutation() {
        let created = new_match("race_to_n", 12).expect("match created");
        let match_id = extract_match_id(&created);

        apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
        let stale =
            apply_action(&match_id, "seat_1", "add-1", 0).expect_err("stale token rejected");
        assert!(stale.contains("\"code\":\"stale_action\""));

        let view = get_view(&match_id, None).expect("view returned");
        assert!(view.contains("\"counter\":1"));
        assert!(view.contains("\"freshness_token\":1"));
    }

    fn extract_match_id(created: &str) -> String {
        created
            .split("\"match_id\":\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("match id is present")
            .to_owned()
    }
}

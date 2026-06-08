use std::{env, fs, path::PathBuf, process, time::Instant};

use column_four::{ColumnFourRandomBot, ColumnFourSeat};
use directional_flip::{DirectionalFlipRandomBot, DirectionalFlipSeat};
use draughts_lite::{DraughtsLiteRandomBot, DraughtsLiteSeat};
use engine_core::{
    Actor, CommandEnvelope, Diagnostic, EffectEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize,
};
use race_to_n::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, RaceEffect,
    RaceRandomBot, RaceSeat, RaceSnapshot, RaceState, SetupOptions,
};
use three_marks::{ThreeMarksRandomBot, ThreeMarksSeat};
use token_bazaar::{TokenBazaarRandomBot, TokenBazaarSeat};

const GAME_ID: &str = "race_to_n";
const GAME_THREE_MARKS: &str = "three_marks";
const GAME_COLUMN_FOUR: &str = "column_four";
const GAME_DIRECTIONAL_FLIP: &str = "directional_flip";
const GAME_DRAUGHTS_LITE: &str = "draughts_lite";
const GAME_HIGH_CARD_DUEL: &str = "high_card_duel";
const GAME_TOKEN_BAZAAR: &str = "token_bazaar";
const RULES_VERSION: u32 = 1;
const DATA_VERSION: u32 = 1;
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const DEFAULT_GAMES: u64 = 1_000;
const DEFAULT_ACTION_CAP: usize = 64;
const BOT_POLICY_VERSION: &str = "race_to_n-random-legal-v1";

fn main() {
    match parse_config(env::args().skip(1)).and_then(run_simulation) {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprint!("{error}");
            process::exit(1);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    game: String,
    games: u64,
    start_seed: u64,
    action_cap: usize,
    inject_failure_seed: Option<u64>,
    failure_report_out: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game: GAME_ID.to_owned(),
            games: DEFAULT_GAMES,
            start_seed: 0,
            action_cap: DEFAULT_ACTION_CAP,
            inject_failure_seed: None,
            failure_report_out: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Summary {
    games_run: u64,
    seat_0_wins: u64,
    seat_1_wins: u64,
    total_actions: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GameOutcome {
    winner: RaceSeat,
    actions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SimulationFailure {
    seed: u64,
    action_cap: usize,
    turn_index: usize,
    action_index: usize,
    actor: String,
    chosen_action_path: String,
    command_stream: Vec<String>,
    state_hash: HashValue,
    effect_hash: HashValue,
    view_hash: HashValue,
    failure_reason: String,
}

struct FailureContext<'a> {
    state: &'a RaceState,
    effects: &'a [EffectEnvelope<RaceEffect>],
    seed: u64,
    action_cap: usize,
    turn_index: usize,
    action_index: usize,
    chosen_action_path: &'a str,
    command_stream: &'a [String],
    failure_reason: &'a str,
}

struct DiagnosticFailureContext<'a> {
    state: &'a RaceState,
    effects: &'a [EffectEnvelope<RaceEffect>],
    seed: u64,
    action_cap: usize,
    action_index: usize,
    actor_seat: RaceSeat,
    chosen_path: &'a str,
    command_stream: &'a [String],
    diagnostic: Diagnostic,
}

fn parse_config(args: impl IntoIterator<Item = String>) -> Result<Config, String> {
    let mut config = Config::default();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Err(help_text()),
            "--version" | "-V" => return Err("simulate 0.1.0\n".to_owned()),
            "--game" => config.game = parse_value(&mut args, "--game")?,
            "--games" => config.games = parse_u64(&mut args, "--games")?,
            "--start-seed" => config.start_seed = parse_u64(&mut args, "--start-seed")?,
            "--action-cap" => config.action_cap = parse_usize(&mut args, "--action-cap")?,
            "--inject-failure-seed" => {
                config.inject_failure_seed = Some(parse_u64(&mut args, "--inject-failure-seed")?)
            }
            "--failure-report-out" => {
                config.failure_report_out = Some(PathBuf::from(parse_value(
                    &mut args,
                    "--failure-report-out",
                )?))
            }
            _ => return Err(format!("unknown argument: {arg}\n\n{}", help_text())),
        }
    }

    if config.game != GAME_ID
        && config.game != GAME_THREE_MARKS
        && config.game != GAME_COLUMN_FOUR
        && config.game != GAME_DIRECTIONAL_FLIP
        && config.game != GAME_DRAUGHTS_LITE
        && config.game != GAME_HIGH_CARD_DUEL
        && config.game != GAME_TOKEN_BAZAAR
    {
        return Err(format!(
            "unsupported game: {}\navailable games: {GAME_ID}, {GAME_THREE_MARKS}, {GAME_COLUMN_FOUR}, {GAME_DIRECTIONAL_FLIP}, {GAME_DRAUGHTS_LITE}, {GAME_HIGH_CARD_DUEL}, {GAME_TOKEN_BAZAAR}\n",
            config.game
        ));
    }
    if config.games == 0 {
        return Err("--games must be greater than 0\n".to_owned());
    }
    if config.action_cap == 0 {
        return Err("--action-cap must be greater than 0\n".to_owned());
    }

    Ok(config)
}

fn parse_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value\n"))
}

fn parse_u64(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<u64, String> {
    parse_value(args, flag)?
        .parse()
        .map_err(|_| format!("{flag} requires an unsigned integer\n"))
}

fn parse_usize(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<usize, String> {
    parse_value(args, flag)?
        .parse()
        .map_err(|_| format!("{flag} requires an unsigned integer\n"))
}

fn help_text() -> String {
    "simulate 0.1.0\n\
         Usage: simulate --game <race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel|token_bazaar> [--games N] [--start-seed N] [--action-cap N] [--failure-report-out PATH]\n\
         Gate 1 native random legal simulation runner.\n"
        .to_owned()
}

fn run_simulation(config: Config) -> Result<String, String> {
    if config.game == GAME_THREE_MARKS {
        return run_three_marks_simulation(config);
    }
    if config.game == GAME_COLUMN_FOUR {
        return run_column_four_simulation(config);
    }
    if config.game == GAME_DIRECTIONAL_FLIP {
        return run_directional_flip_simulation(config);
    }
    if config.game == GAME_DRAUGHTS_LITE {
        return run_draughts_lite_simulation(config);
    }
    if config.game == GAME_HIGH_CARD_DUEL {
        return run_high_card_duel_simulation(config);
    }
    if config.game == GAME_TOKEN_BAZAAR {
        return run_token_bazaar_simulation(config);
    }

    let started = Instant::now();
    let mut summary = Summary::default();

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        match run_one_game(&config, seed) {
            Ok(outcome) => {
                summary.games_run += 1;
                summary.total_actions += outcome.actions as u64;
                match outcome.winner {
                    RaceSeat::Seat0 => summary.seat_0_wins += 1,
                    RaceSeat::Seat1 => summary.seat_1_wins += 1,
                }
            }
            Err(failure) => {
                if let Some(path) = &config.failure_report_out {
                    fs::write(path, failure_report_json(&config, &failure)).map_err(|error| {
                        format!(
                            "failed to write failure report `{}`: {error}\n",
                            path.display()
                        )
                    })?;
                }
                return Err(format_failure(&failure));
            }
        }
    }

    Ok(format_summary(
        &config,
        &summary,
        started.elapsed().as_secs_f64(),
    ))
}

fn run_token_bazaar_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut draws = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_token_bazaar_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(TokenBazaarSeat::Seat0) => seat_0_wins += 1,
            Some(TokenBazaarSeat::Seat1) => seat_1_wins += 1,
            None => draws += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=token_bazaar\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         draws={draws}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_high_card_duel_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut draws = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_high_card_duel_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(high_card_duel::HighCardDuelSeat::Seat0) => seat_0_wins += 1,
            Some(high_card_duel::HighCardDuelSeat::Seat1) => seat_1_wins += 1,
            None => draws += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=high_card_duel\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         draws={draws}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_draughts_lite_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut bounded_nonterminal_at_cap = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_draughts_lite_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(DraughtsLiteSeat::Seat0) => seat_0_wins += 1,
            Some(DraughtsLiteSeat::Seat1) => seat_1_wins += 1,
            None => bounded_nonterminal_at_cap += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=draughts_lite\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         bounded_nonterminal_at_cap={bounded_nonterminal_at_cap}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_column_four_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut draws = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_column_four_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(ColumnFourSeat::Seat0) => seat_0_wins += 1,
            Some(ColumnFourSeat::Seat1) => seat_1_wins += 1,
            None => draws += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=column_four\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         draws={draws}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_directional_flip_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut draws = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_directional_flip_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(DirectionalFlipSeat::Seat0) => seat_0_wins += 1,
            Some(DirectionalFlipSeat::Seat1) => seat_1_wins += 1,
            None => draws += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=directional_flip\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         draws={draws}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_three_marks_simulation(config: Config) -> Result<String, String> {
    let started = Instant::now();
    let mut games_run = 0_u64;
    let mut seat_0_wins = 0_u64;
    let mut seat_1_wins = 0_u64;
    let mut draws = 0_u64;
    let mut total_actions = 0_u64;

    for offset in 0..config.games {
        let seed = config.start_seed.wrapping_add(offset);
        let (outcome, actions) = run_one_three_marks_game(&config, seed)?;
        games_run += 1;
        total_actions += actions as u64;
        match outcome {
            Some(ThreeMarksSeat::Seat0) => seat_0_wins += 1,
            Some(ThreeMarksSeat::Seat1) => seat_1_wins += 1,
            None => draws += 1,
        }
    }

    let elapsed_secs = started.elapsed().as_secs_f64();
    let average_length = total_actions as f64 / games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        games_run as f64 / elapsed_secs
    } else {
        games_run as f64
    };
    Ok(format!(
        "simulate summary\n\
         game_id=three_marks\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={games_run}\n\
         seat_0_wins={seat_0_wins}\n\
         seat_1_wins={seat_1_wins}\n\
         draws={draws}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.start_seed
    ))
}

fn run_one_three_marks_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<ThreeMarksSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        three_marks::setup_match(Seed(seed), &seats, &three_marks::SetupOptions::default())
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    for action_index in 0..config.action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((three_marks_winner(outcome), action_index));
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = ThreeMarksRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = three_marks::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        three_marks::apply_action(&mut state, validated);
    }

    Err(format!(
        "SIMULATION FAILURE\n\
         game_id=three_marks\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={seed}\n\
         action_cap={}\n\
         failure_reason=action cap reached before terminal outcome\n\
         replay_command=cargo run -p simulate -- --game three_marks --games 1 --start-seed {seed} --action-cap {}\n",
        config.action_cap, config.action_cap
    ))
}

fn three_marks_winner(outcome: three_marks::TerminalOutcome) -> Option<ThreeMarksSeat> {
    match outcome {
        three_marks::TerminalOutcome::Win { seat, .. } => Some(seat),
        three_marks::TerminalOutcome::Draw => None,
    }
}

fn run_one_column_four_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<ColumnFourSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        column_four::setup_match(Seed(seed), &seats, &column_four::SetupOptions::default())
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    for action_index in 0..config.action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((column_four_winner(outcome), action_index));
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = ColumnFourRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = column_four::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        column_four::apply_action(&mut state, validated);
    }

    Err(format!(
        "SIMULATION FAILURE\n\
         game_id=column_four\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={seed}\n\
         action_cap={}\n\
         failure_reason=action cap reached before terminal outcome\n\
         replay_command=cargo run -p simulate -- --game column_four --games 1 --start-seed {seed} --action-cap {}\n",
        config.action_cap, config.action_cap
    ))
}

fn column_four_winner(outcome: column_four::TerminalOutcome) -> Option<ColumnFourSeat> {
    match outcome {
        column_four::TerminalOutcome::Win { seat, .. } => Some(seat),
        column_four::TerminalOutcome::Draw => None,
    }
}

fn run_one_directional_flip_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<DirectionalFlipSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state = directional_flip::setup_match(
        Seed(seed),
        &seats,
        &directional_flip::SetupOptions::default(),
    )
    .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    let action_cap = config.action_cap.max(128);
    for action_index in 0..action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((directional_flip_winner(outcome), action_index));
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = DirectionalFlipRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = directional_flip::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        directional_flip::apply_action(&mut state, validated);
    }

    Err(format!(
        "SIMULATION FAILURE\n\
         game_id=directional_flip\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={seed}\n\
         action_cap={}\n\
         failure_reason=action cap reached before terminal outcome\n\
         replay_command=cargo run -p simulate -- --game directional_flip --games 1 --start-seed {seed} --action-cap {action_cap}\n",
        action_cap
    ))
}

fn directional_flip_winner(
    outcome: directional_flip::TerminalOutcome,
) -> Option<DirectionalFlipSeat> {
    match outcome {
        directional_flip::TerminalOutcome::Win { seat } => Some(seat),
        directional_flip::TerminalOutcome::Draw => None,
    }
}

fn run_one_draughts_lite_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<DraughtsLiteSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        draughts_lite::setup_match(Seed(seed), &seats, &draughts_lite::SetupOptions::default())
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    for action_index in 0..config.action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((Some(draughts_lite_winner(outcome)), action_index));
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = DraughtsLiteRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = draughts_lite::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        draughts_lite::apply_action(&mut state, validated);
    }

    Ok((None, config.action_cap))
}

fn run_one_high_card_duel_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<high_card_duel::HighCardDuelSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        high_card_duel::setup_match(Seed(seed), &seats, &high_card_duel::SetupOptions::default())
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    for action_index in 0..config.action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((high_card_duel_winner(outcome), action_index));
        }

        let actor_seat = high_card_duel::active_commit_seat(&state)
            .ok_or_else(|| "non-terminal state has no active commit seat".to_owned())?;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = high_card_duel::HighCardDuelRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = high_card_duel::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        high_card_duel::apply_action(&mut state, validated);
    }

    Err(format!(
        "SIMULATION FAILURE\n\
         game_id=high_card_duel\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={seed}\n\
         action_cap={}\n\
         failure_reason=action cap reached before terminal outcome\n\
         replay_command=cargo run -p simulate -- --game high_card_duel --games 1 --start-seed {seed} --action-cap {}\n",
        config.action_cap, config.action_cap
    ))
}

fn run_one_token_bazaar_game(
    config: &Config,
    seed: u64,
) -> Result<(Option<TokenBazaarSeat>, usize), String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        token_bazaar::setup_match(Seed(seed), &seats, &token_bazaar::SetupOptions::default())
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;

    for action_index in 0..config.action_cap {
        if let Some(outcome) = state.terminal_outcome {
            return Ok((token_bazaar_winner(outcome), action_index));
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot = TokenBazaarRandomBot::new(Seed(bot_seed(seed, action_index)));
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = token_bazaar::validate_command(&state, &command)
            .map_err(|diagnostic| format!("{}: {}", diagnostic.code, diagnostic.message))?;
        token_bazaar::apply_action(&mut state, validated);
    }

    Err(format!(
        "SIMULATION FAILURE\n\
         game_id=token_bazaar\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={seed}\n\
         action_cap={}\n\
         failure_reason=action cap reached before terminal outcome\n\
         replay_command=cargo run -p simulate -- --game token_bazaar --games 1 --start-seed {seed} --action-cap {}\n",
        config.action_cap, config.action_cap
    ))
}

fn high_card_duel_winner(
    outcome: high_card_duel::TerminalOutcome,
) -> Option<high_card_duel::HighCardDuelSeat> {
    match outcome {
        high_card_duel::TerminalOutcome::Win { seat } => Some(seat),
        high_card_duel::TerminalOutcome::Draw => None,
    }
}

fn token_bazaar_winner(outcome: token_bazaar::TerminalOutcome) -> Option<TokenBazaarSeat> {
    match outcome {
        token_bazaar::TerminalOutcome::Win { seat } => Some(seat),
        token_bazaar::TerminalOutcome::Draw => None,
    }
}

fn draughts_lite_winner(outcome: draughts_lite::TerminalOutcome) -> DraughtsLiteSeat {
    match outcome {
        draughts_lite::TerminalOutcome::Win { seat } => seat,
    }
}

fn run_one_game(config: &Config, seed: u64) -> Result<GameOutcome, Box<SimulationFailure>> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state =
        setup_match(Seed(seed), &seats, &SetupOptions::default()).map_err(|diagnostic| {
            Box::new(failure_from_diagnostic(seed, config.action_cap, diagnostic))
        })?;
    let mut command_stream = Vec::new();
    let mut effects = Vec::new();

    if config.inject_failure_seed == Some(seed) {
        return Err(Box::new(build_failure(FailureContext {
            state: &state,
            effects: &effects,
            seed,
            action_cap: config.action_cap,
            turn_index: 0,
            action_index: 0,
            chosen_action_path: "none",
            command_stream: &command_stream,
            failure_reason: "injected failure",
        })));
    }

    for action_index in 0..config.action_cap {
        if let Some(winner) = state.winner {
            return Ok(GameOutcome {
                winner,
                actions: action_index,
            });
        }

        let actor_seat = state.active_seat;
        let actor = Actor {
            seat_id: state.seats[actor_seat.index()].clone(),
        };
        let bot_seed = Seed(bot_seed(seed, action_index));
        let bot = RaceRandomBot::new(bot_seed);
        let action_path = bot
            .select_action(&state, actor_seat)
            .map_err(|diagnostic| {
                Box::new(diagnostic_failure(DiagnosticFailureContext {
                    state: &state,
                    effects: &effects,
                    seed,
                    action_cap: config.action_cap,
                    action_index,
                    actor_seat,
                    chosen_path: "none",
                    command_stream: &command_stream,
                    diagnostic,
                }))
            })?;
        let chosen_path = format_action_path(&action_path.segments);
        let command = CommandEnvelope {
            actor,
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let validated = validate_command(&state, &command).map_err(|diagnostic| {
            Box::new(diagnostic_failure(DiagnosticFailureContext {
                state: &state,
                effects: &effects,
                seed,
                action_cap: config.action_cap,
                action_index,
                actor_seat,
                chosen_path: &chosen_path,
                command_stream: &command_stream,
                diagnostic,
            }))
        })?;
        command_stream.push(format!("{}:{chosen_path}", actor_seat.as_str()));
        effects.extend(apply_action(&mut state, validated));

        if let Err(reason) = check_invariants(&state, actor_seat) {
            return Err(Box::new(build_failure(FailureContext {
                state: &state,
                effects: &effects,
                seed,
                action_cap: config.action_cap,
                turn_index: action_index,
                action_index,
                chosen_action_path: &chosen_path,
                command_stream: &command_stream,
                failure_reason: &reason,
            })));
        }
    }

    Err(Box::new(build_failure(FailureContext {
        state: &state,
        effects: &effects,
        seed,
        action_cap: config.action_cap,
        turn_index: config.action_cap,
        action_index: config.action_cap,
        chosen_action_path: "none",
        command_stream: &command_stream,
        failure_reason: "action cap reached before terminal outcome",
    })))
}

fn diagnostic_failure(input: DiagnosticFailureContext<'_>) -> SimulationFailure {
    let reason = format!("{}: {}", input.diagnostic.code, input.diagnostic.message);
    build_failure(FailureContext {
        state: input.state,
        effects: input.effects,
        seed: input.seed,
        action_cap: input.action_cap,
        turn_index: input.action_index,
        action_index: input.action_index,
        chosen_action_path: input.chosen_path,
        command_stream: input.command_stream,
        failure_reason: &reason,
    })
    .with_actor(input.actor_seat.as_str())
}

fn failure_from_diagnostic(
    seed: u64,
    action_cap: usize,
    diagnostic: Diagnostic,
) -> SimulationFailure {
    SimulationFailure {
        seed,
        action_cap,
        turn_index: 0,
        action_index: 0,
        actor: "none".to_owned(),
        chosen_action_path: "none".to_owned(),
        command_stream: Vec::new(),
        state_hash: HashValue(0),
        effect_hash: HashValue(0),
        view_hash: HashValue(0),
        failure_reason: format!("{}: {}", diagnostic.code, diagnostic.message),
    }
}

fn build_failure(input: FailureContext<'_>) -> SimulationFailure {
    SimulationFailure {
        seed: input.seed,
        action_cap: input.action_cap,
        turn_index: input.turn_index,
        action_index: input.action_index,
        actor: input.state.active_seat.as_str().to_owned(),
        chosen_action_path: input.chosen_action_path.to_owned(),
        command_stream: input.command_stream.to_vec(),
        state_hash: RaceSnapshot::from_state(input.state).stable_hash(),
        effect_hash: effect_hash(input.effects),
        view_hash: project_view(input.state).stable_hash(),
        failure_reason: input.failure_reason.to_owned(),
    }
}

trait WithActor {
    fn with_actor(self, actor: &str) -> Self;
}

impl WithActor for SimulationFailure {
    fn with_actor(mut self, actor: &str) -> Self {
        self.actor = actor.to_owned();
        self
    }
}

fn check_invariants(state: &RaceState, actor_seat: RaceSeat) -> Result<(), String> {
    if state.counter.0 > state.variant.target {
        return Err("counter exceeded target".to_owned());
    }
    if state.winner.is_some() && state.counter.0 != state.variant.target {
        return Err("winner set before target reached".to_owned());
    }
    if state.winner == Some(actor_seat) && state.active_seat != actor_seat {
        return Err("terminal winner changed active seat".to_owned());
    }
    if state.winner.is_none() {
        let actor = Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        };
        let tree = legal_action_tree(state, &actor);
        if tree.freshness_token != state.freshness_token {
            return Err("legal tree freshness token diverged from state".to_owned());
        }
        if tree.root.choices.is_empty() {
            return Err("non-terminal state has no legal action".to_owned());
        }
    }
    Ok(())
}

fn bot_seed(seed: u64, action_index: usize) -> u64 {
    seed.wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add(action_index as u64)
}

fn effect_hash(effects: &[EffectEnvelope<RaceEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(format_effect)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn format_effect(effect: &EffectEnvelope<RaceEffect>) -> String {
    let visibility = match &effect.visibility {
        engine_core::VisibilityScope::Public => "public".to_owned(),
        engine_core::VisibilityScope::PrivateToSeat(seat) => format!("private:{}", seat.0),
    };
    let payload = match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => {
            format!("action_started:{}:{amount}", actor.as_str())
        }
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "counter_advanced:{}:{}:{}:{amount}",
            actor.as_str(),
            from.0,
            to.0
        ),
        RaceEffect::TurnChanged { next_actor } => format!("turn_changed:{}", next_actor.as_str()),
        RaceEffect::GameEnded { winner } => format!("game_ended:{}", winner.as_str()),
        RaceEffect::ActionCompleted { actor } => format!("action_completed:{}", actor.as_str()),
    };
    format!("{visibility}:{payload}")
}

fn format_action_path(segments: &[String]) -> String {
    if segments.is_empty() {
        "none".to_owned()
    } else {
        segments.join("/")
    }
}

fn format_summary(config: &Config, summary: &Summary, elapsed_secs: f64) -> String {
    let average_length = summary.total_actions as f64 / summary.games_run as f64;
    let throughput = if elapsed_secs > 0.0 {
        summary.games_run as f64 / elapsed_secs
    } else {
        summary.games_run as f64
    };
    format!(
        "simulate summary\n\
         game_id={}\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         start_seed={}\n\
         games_run={}\n\
         seat_0_wins={}\n\
         seat_1_wins={}\n\
         average_length={average_length:.2}\n\
         throughput_games_per_sec={throughput:.2}\n",
        config.game, config.start_seed, summary.games_run, summary.seat_0_wins, summary.seat_1_wins
    )
}

fn format_failure(failure: &SimulationFailure) -> String {
    let command_stream = if failure.command_stream.is_empty() {
        "[]".to_owned()
    } else {
        failure.command_stream.join(",")
    };
    format!(
        "SIMULATION FAILURE\n\
         game_id={GAME_ID}\n\
         rules_version={RULES_VERSION}\n\
         data_version={DATA_VERSION}\n\
         seed={}\n\
         options=variant=race_to_21 action_cap={}\n\
         bot_policy_versions={BOT_POLICY_VERSION}\n\
         turn_index={}\n\
         action_index={}\n\
         actor={}\n\
         chosen_action_path={}\n\
         command_stream={command_stream}\n\
         state_hash={}\n\
         effect_hash={}\n\
         view_hash={}\n\
         failure_reason={}\n\
         replay_command=cargo run -p simulate -- --game {GAME_ID} --games 1 --start-seed {} --action-cap {}\n",
        failure.seed,
        failure.action_cap,
        failure.turn_index,
        failure.action_index,
        failure.actor,
        failure.chosen_action_path,
        failure.state_hash.0,
        failure.effect_hash.0,
        failure.view_hash.0,
        failure.failure_reason,
        failure.seed,
        failure.action_cap
    )
}

fn failure_report_json(config: &Config, failure: &SimulationFailure) -> String {
    let commands = failure
        .command_stream
        .iter()
        .map(|command| format!("\"{}\"", escape_json(command)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{\n",
            "  \"schema_version\": 1,\n",
            "  \"report_kind\": \"simulation_failure\",\n",
            "  \"game_id\": \"{}\",\n",
            "  \"rules_version\": \"race_to_n-rules-v{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"seed\": {},\n",
            "  \"start_seed\": {},\n",
            "  \"games_requested\": {},\n",
            "  \"options\": {{\"variant\": \"race_to_21\", \"action_cap\": {}}},\n",
            "  \"variant\": \"race_to_21\",\n",
            "  \"bot_policy_versions\": [\"{}\"],\n",
            "  \"turn_index\": {},\n",
            "  \"action_index\": {},\n",
            "  \"actor\": \"{}\",\n",
            "  \"chosen_action_path\": \"{}\",\n",
            "  \"command_stream\": [{}],\n",
            "  \"state_hash\": \"{}\",\n",
            "  \"effect_hash\": \"{}\",\n",
            "  \"view_hash\": \"{}\",\n",
            "  \"failure_reason\": \"{}\",\n",
            "  \"replay_command\": \"{}\"\n",
            "}}\n"
        ),
        GAME_ID,
        RULES_VERSION,
        DATA_VERSION,
        ENGINE_VERSION,
        failure.seed,
        config.start_seed,
        config.games,
        failure.action_cap,
        BOT_POLICY_VERSION,
        failure.turn_index,
        failure.action_index,
        escape_json(&failure.actor),
        escape_json(&failure.chosen_action_path),
        commands,
        failure.state_hash.0,
        failure.effect_hash.0,
        failure.view_hash.0,
        escape_json(&failure.failure_reason),
        escape_json(&format!(
            "cargo run -p simulate -- --game {GAME_ID} --games 1 --start-seed {} --action-cap {}",
            failure.seed, failure.action_cap
        ))
    )
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_seed_run_completes_and_checks_invariants() {
        let output = run_simulation(Config {
            games: 1_000,
            ..Config::default()
        })
        .expect("simulation succeeds");

        assert!(output.contains("games_run=1000"));
        assert!(output.contains("average_length="));
        assert!(output.contains("throughput_games_per_sec="));
    }

    #[test]
    fn failure_formatting_includes_parseable_replay_command() {
        let error = run_simulation(Config {
            games: 1,
            start_seed: 7,
            inject_failure_seed: Some(7),
            ..Config::default()
        })
        .expect_err("injected failure emits failure block");

        assert!(error.contains("SIMULATION FAILURE"));
        assert!(error.contains("game_id=race_to_n"));
        assert!(error.contains("seed=7"));
        assert!(error.contains(
            "replay_command=cargo run -p simulate -- --game race_to_n --games 1 --start-seed 7 --action-cap 64"
        ));
    }

    #[test]
    fn failure_report_json_carries_seed_reducer_contract_fields() {
        let config = Config {
            games: 1,
            start_seed: 7,
            inject_failure_seed: Some(7),
            ..Config::default()
        };
        let failure = run_one_game(&config, 7).expect_err("injected failure");
        let report = failure_report_json(&config, &failure);

        assert!(report.contains("\"report_kind\": \"simulation_failure\""));
        assert!(report.contains("\"game_id\": \"race_to_n\""));
        assert!(report.contains("\"rules_version\": \"race_to_n-rules-v1\""));
        assert!(report.contains("\"engine_version\": \"engine-core-0.1.0\""));
        assert!(report.contains("\"seed\": 7"));
        assert!(report.contains("\"variant\": \"race_to_21\""));
        assert!(report.contains("\"command_stream\""));
        assert!(report.contains("\"state_hash\""));
        assert!(report.contains("\"effect_hash\""));
        assert!(report.contains("\"view_hash\""));
        assert!(report.contains("\"failure_reason\": \"injected failure\""));
        assert!(report.contains("\"replay_command\""));
    }
}

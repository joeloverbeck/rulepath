import React, { useCallback, useEffect, useMemo, useReducer, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";
import { AppShell } from "./components/AppShell";
import { ActionControls } from "./components/ActionControls";
import { BlackglassPactBoard } from "./components/BlackglassPactBoard";
import { BriarCircuitBoard } from "./components/BriarCircuitBoard";
import { ColumnFourBoard } from "./components/ColumnFourBoard";
import { DevPanel } from "./components/DevPanel";
import { DirectionalFlipBoard } from "./components/DirectionalFlipBoard";
import { DraughtsLiteBoard } from "./components/DraughtsLiteBoard";
import { EffectLog } from "./components/EffectLog";
import { summarizeEffect, useReducedMotionPreference } from "./components/effectFeedback";
import { EventFrontierBoard } from "./components/EventFrontierBoard";
import { FloodWatchBoard } from "./components/FloodWatchBoard";
import { FrontierControlBoard } from "./components/FrontierControlBoard";
import { GamePicker } from "./components/GamePicker";
import { HighCardDuelBoard } from "./components/HighCardDuelBoard";
import { MatchSetup, riverLedgerShortStackPreset } from "./components/MatchSetup";
import { MaskedClaimsBoard } from "./components/MaskedClaimsBoard";
import { ModeControls } from "./components/ModeControls";
import { PlainTricksBoard } from "./components/PlainTricksBoard";
import { PokerLiteBoard } from "./components/PokerLiteBoard";
import { RaceBoard } from "./components/RaceBoard";
import { ReplayImportExport } from "./components/ReplayImportExport";
import { ReplayViewer } from "./components/ReplayViewer";
import { RiverLedgerBoard } from "./components/RiverLedgerBoard";
import { SecretDraftBoard } from "./components/SecretDraftBoard";
import { SeatFrame, type SeatFrameViewerMode } from "./components/SeatFrame";
import { ThreeMarksBoard } from "./components/ThreeMarksBoard";
import { TokenBazaarBoard } from "./components/TokenBazaarBoard";
import { TurnReportPanel } from "./components/TurnReportPanel";
import { VowTideBoard } from "./components/VowTideBoard";
import { animationRegistry } from "./animation/registry";
import { EffectAnimationScheduler } from "./animation/scheduler";
import { createDevSettleAssertion } from "./animation/settleAssertion";
import { initialShellState, shellReducer, type RefreshPayload, type SetupPlayMode } from "./state/shellReducer";
import {
  loadApi,
  type ActionChoice,
  type ApiError,
  type BlackglassPactPublicView,
  type BriarCircuitPublicView,
  type ColumnFourPublicView,
  type DirectionalFlipPublicView,
  type DraughtsLitePublicView,
  type EventFrontierPublicView,
  type FloodWatchPublicView,
  type FrontierControlPublicView,
  type GameCatalogEntry,
  type HighCardDuelPublicView,
  type MaskedClaimsPublicView,
  type PlainTricksPublicView,
  type PokerLitePublicView,
  type PublicView,
  type RacePublicView,
  type ReplayExportDocument,
  type RiverLedgerPublicView,
  type RulepathApi,
  type SeatId,
  type SeatDisplayLabel,
  type SecretDraftPublicView,
  type ThreeMarksPublicView,
  type TokenBazaarPublicView,
  type VowTidePublicView,
  type ViewerSeatId,
  type ViewerMode,
} from "./wasm/client";

type AppTextState = {
  mode: "loading" | "ready" | "playing" | "error";
  version: string;
  matchId: string | null;
  view:
    | {
        game_id: string;
        variant_id?: string;
        active_seat: ViewerSeatId;
        freshness_token: number;
        status: string;
      }
    | null;
  choices: string[];
  effects: string[];
  diagnostic: ApiError | null;
};

function App() {
  const [state, dispatch] = useReducer(shellReducer, initialShellState);
  const motion = useReducedMotionPreference();
  const { api, version, matchId, view, actionTree, effects, effectCursor, diagnostic, staleToken } = state;
  const selectedGame = state.catalog.find((game) => game.game_id === state.selectedGameId) ?? null;
  const latestEffect = effects.at(-1) ?? null;
  const humanActorSeat = view ? humanSeatForMode(state.setup.playMode, view) : null;
  const schedulerRef = useRef<EffectAnimationScheduler | null>(null);
  const lastAnimatedCursorRef = useRef(0);
  const autoBotInFlightRef = useRef(false);
  const [schedulerActive, setSchedulerActive] = useState(false);
  const [autoBotInFlight, setAutoBotInFlight] = useState(false);
  const autoplayInFlightRef = useRef(false);
  const activeGameIdRef = useRef(state.selectedGameId);
  const activeViewRef = useRef(view);
  const orchestrationPausedRef = useRef(state.orchestration.paused);
  const autoplayRunningRef = useRef(state.autoplay.running);

  if (!schedulerRef.current) {
    schedulerRef.current = new EffectAnimationScheduler({
      reducedMotion: state.reducedMotion,
      presenter: (step) =>
        animationRegistry.resolve(activeGameIdRef.current ?? "", step, {
          root: document,
          reducedMotion: step.reducedMotion,
        }),
      settle: createDevSettleAssertion(() => activeViewRef.current),
    });
  }

  const refresh = useCallback(
    (loadedApi: NonNullable<typeof api>, loadedMatchId: string, sinceCursor: number, viewerOverride?: ViewerMode) => {
      const viewerMode =
        viewerOverride ?? effectiveViewerMode(loadedApi, loadedMatchId, state.setup.playMode, state.viewerMode);
      const nextView = loadedApi.getView(loadedMatchId, viewerMode);
      const nextEffects = loadedApi.getEffects(loadedMatchId, sinceCursor, viewerMode);
      const newestCursor = nextEffects.reduce((cursor, entry) => Math.max(cursor, entry.cursor), sinceCursor);
      const nextActorSeat = humanSeatForMode(state.setup.playMode, nextView);
      const nextTree =
        nextActorSeat && !isTerminalView(nextView)
          ? loadedApi.getActionTree(loadedMatchId, nextActorSeat, { kind: "seat", seat: nextActorSeat })
          : { freshness_token: nextView.freshness_token, choices: [] };

      const payload: RefreshPayload = {
        view: nextView,
        actionTree: nextTree,
        effects: nextEffects,
        effectCursor: newestCursor,
        viewerMode,
      };
      dispatch({ type: "refreshed", payload });
    },
    [state.setup.playMode, state.viewerMode],
  );

  useEffect(() => {
    let cancelled = false;

    loadApi()
      .then((loadedApi) => {
        if (cancelled) {
          return;
        }
        dispatch({
          type: "wasmLoaded",
          api: loadedApi,
          version: loadedApi.version(),
          catalog: loadedApi.listGames(),
          featureReport: loadedApi.featureReport(),
        });
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          dispatch({
            type: "wasmLoadFailed",
            message: error instanceof Error ? error.message : "Unable to load wasm-api artifact",
          });
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    dispatch({ type: "reducedMotionChanged", reducedMotion: motion.reducedMotion });
    schedulerRef.current?.setReducedMotion(motion.reducedMotion);
  }, [motion.reducedMotion]);

  useEffect(() => schedulerRef.current?.subscribeActivity(setSchedulerActive), []);

  useEffect(() => {
    activeGameIdRef.current = state.selectedGameId;
  }, [state.selectedGameId]);

  useEffect(() => {
    activeViewRef.current = view;
  }, [view]);

  useEffect(() => {
    schedulerRef.current?.setRate(state.orchestration.rate);
  }, [state.orchestration.rate]);

  useEffect(() => {
    orchestrationPausedRef.current = state.orchestration.paused;
  }, [state.orchestration.paused]);

  useEffect(() => {
    autoplayRunningRef.current = state.autoplay.running;
  }, [state.autoplay.running]);

  useEffect(() => {
    lastAnimatedCursorRef.current = 0;
    autoBotInFlightRef.current = false;
    setAutoBotInFlight(false);
    autoplayInFlightRef.current = false;
    void schedulerRef.current?.flush();
  }, [matchId]);

  const flushScheduler = useCallback(async () => {
    await schedulerRef.current?.flush();
  }, []);

  const start = useCallback(() => {
    if (!api || !state.selectedGameId) {
      return;
    }
    dispatch({ type: "matchStarting" });
    const seatCount = state.setup.seatCount ?? selectedGame?.default_seats;
    const riverLedgerOptions = riverLedgerSetupOptions(state.selectedGameId, state.setup.riverLedgerStackMode, state.setup.riverLedgerStacks, seatCount);
    const created =
      riverLedgerOptions && typeof seatCount === "number"
        ? api.newMatchWithOptions(state.selectedGameId, state.setup.seed, seatCount, riverLedgerOptions)
        : api.newMatch(state.selectedGameId, state.setup.seed, selectedVariantForStart(selectedGame, state.setup.variantId), seatCount);
    dispatch({ type: "matchStarted", matchId: created.match_id });
    refresh(api, created.match_id, 0);
  }, [
    api,
    refresh,
    selectedGame,
    state.selectedGameId,
    state.setup.riverLedgerStackMode,
    state.setup.riverLedgerStacks,
    state.setup.seatCount,
    state.setup.seed,
    state.setup.variantId,
  ]);

  const playChoice = useCallback(
    async (choice: ActionChoice) => {
      if (!api || !matchId || !view) {
        return;
      }
      await flushScheduler();
      const actorSeat = humanSeatForMode(state.setup.playMode, view);
      if (!actorSeat) {
        return;
      }
      dispatch({ type: "diagnosticCleared" });
      const tokenBeforeAction = view.freshness_token;
      try {
        api.applyAction(matchId, actorSeat, choice.segment, tokenBeforeAction);
        dispatch({ type: "actionApplied", staleToken: tokenBeforeAction });
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      }
    },
    [api, effectCursor, flushScheduler, matchId, refresh, state.setup.playMode, view],
  );

  const playPath = useCallback(
    async (path: string[]) => {
      if (!api || !matchId || !view) {
        return;
      }
      await flushScheduler();
      const actorSeat = humanSeatForMode(state.setup.playMode, view);
      if (!actorSeat || path.length === 0) {
        return;
      }
      dispatch({ type: "diagnosticCleared" });
      const tokenBeforeAction = view.freshness_token;
      try {
        api.applyActionPath(matchId, actorSeat, path, tokenBeforeAction);
        dispatch({ type: "actionApplied", staleToken: tokenBeforeAction });
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      }
    },
    [api, effectCursor, flushScheduler, matchId, refresh, state.setup.playMode, view],
  );

  const runBotStep = useCallback(() => {
    if (
      !api ||
      !matchId ||
      !view ||
      isTerminalView(view) ||
      !view.active_seat ||
      !botSeatForMode(state.setup.playMode, view.active_seat)
    ) {
      return;
    }
    dispatch({ type: "botTurnStarted" });
    try {
      const botResult = api.runBotTurn(matchId, view.active_seat, botSeed(view));
      dispatch({ type: "botTurnCompleted", result: botResult });
      refresh(api, matchId, effectCursor);
    } catch (error: unknown) {
      dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
    }
  }, [api, effectCursor, matchId, refresh, state.setup.playMode, view]);

  useEffect(() => {
    if (
      !api ||
      !matchId ||
      !view ||
      state.setup.playMode !== "human_vs_bot" ||
      state.orchestration.paused ||
      state.pendingOperation !== null ||
      isTerminalView(view) ||
      !view.active_seat ||
      !botSeatForMode(state.setup.playMode, view.active_seat) ||
      autoBotInFlightRef.current
    ) {
      return;
    }

    const botSeat = view.active_seat;
    autoBotInFlightRef.current = true;
    setAutoBotInFlight(true);
    const effectsToDrain = effects.filter((entry) => entry.cursor > lastAnimatedCursorRef.current);
    const newestCursor = effectsToDrain.reduce((cursor, entry) => Math.max(cursor, entry.cursor), lastAnimatedCursorRef.current);

    void (async () => {
      try {
        await schedulerRef.current?.enqueueEffects(effectsToDrain);
        lastAnimatedCursorRef.current = newestCursor;
        if (orchestrationPausedRef.current) {
          return;
        }
        dispatch({ type: "botTurnStarted" });
        const botResult = api.runBotTurn(matchId, botSeat, botSeed(view));
        dispatch({ type: "botTurnCompleted", result: botResult });
        refresh(api, matchId, effectCursor);
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
      } finally {
        autoBotInFlightRef.current = false;
        setAutoBotInFlight(false);
      }
    })();
  }, [api, effectCursor, effects, matchId, refresh, state.orchestration.paused, state.pendingOperation, state.setup.playMode, view]);

  useEffect(() => {
    if (state.setup.playMode !== "hotseat" || state.pendingOperation !== null || !view) {
      return;
    }
    const effectsToDrain = effects.filter((entry) => entry.cursor > lastAnimatedCursorRef.current);
    if (effectsToDrain.length === 0) {
      return;
    }
    const newestCursor = effectsToDrain.reduce((cursor, entry) => Math.max(cursor, entry.cursor), lastAnimatedCursorRef.current);

    void (async () => {
      await schedulerRef.current?.enqueueEffects(effectsToDrain);
      lastAnimatedCursorRef.current = newestCursor;
    })();
  }, [effects, state.pendingOperation, state.setup.playMode, view]);

  const changeViewerMode = useCallback(
    (viewerMode: ViewerMode) => {
      dispatch({ type: "viewerModeChanged", viewerMode });
      if (api && matchId) {
        refresh(api, matchId, effectCursor, viewerMode);
      }
    },
    [api, effectCursor, matchId, refresh],
  );

  const changeSeatFrameViewerMode = useCallback(
    (viewerMode: SeatFrameViewerMode) => {
      if (viewerMode.kind === "observer") {
        changeViewerMode(viewerMode);
        return;
      }
      const activeSeats = activeViewerSeatIds(view, selectedGame);
      if (activeSeats.has(viewerMode.seat) && isViewerSeatId(viewerMode.seat)) {
        changeViewerMode({ kind: "seat", seat: viewerMode.seat });
        return;
      }
      changeViewerMode({ kind: "observer" });
    },
    [changeViewerMode, selectedGame, view],
  );

  useEffect(() => {
    if (state.viewerMode.kind === "seat" && !activeViewerSeatIds(view, selectedGame).has(state.viewerMode.seat)) {
      changeViewerMode({ kind: "observer" });
    }
  }, [changeViewerMode, selectedGame, state.viewerMode, view]);

  const exportCurrentReplay = useCallback(() => {
    if (!api || !matchId) {
      throw { code: "no_match", message: "Start a match before exporting a replay." } satisfies ApiError;
    }
    return api.exportReplay(matchId);
  }, [api, matchId]);

  const importReplay = useCallback(
    (documentText: string) => {
      if (!api) {
        throw { code: "wasm_not_ready", message: "WASM API is not ready." } satisfies ApiError;
      }
      dispatch({ type: "pendingOperationChanged", pendingOperation: "importReplay" });
      try {
        const parsedDocument = parseReplayDocument(documentText);
        const imported = api.importReplay(documentText);
        const step = api.replayReset(imported.replay_id);
        dispatch({ type: "replayImported", replayId: imported.replay_id, document: parsedDocument, step });
      } catch (error: unknown) {
        dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
        throw error;
      }
    },
    [api],
  );

  const stepReplay = useCallback(async () => {
    if (!api || !state.replay) {
      return;
    }
    await flushScheduler();
    dispatch({ type: "pendingOperationChanged", pendingOperation: "stepReplay" });
    const step = api.replayStep(state.replay.replayId, state.replay.cursor + 1);
    dispatch({ type: "replayStepped", step });
  }, [api, flushScheduler, state.replay]);

  const resetReplay = useCallback(async () => {
    if (!api || !state.replay) {
      return;
    }
    await flushScheduler();
    dispatch({ type: "pendingOperationChanged", pendingOperation: "stepReplay" });
    const step = api.replayReset(state.replay.replayId);
    dispatch({ type: "replayReset", step });
  }, [api, flushScheduler, state.replay]);

  const openRules = useCallback((gameId: string) => {
    dispatch({ type: "rulesPanelOpened", gameId });
  }, []);

  const closeRules = useCallback(() => {
    dispatch({ type: "rulesPanelClosed" });
  }, []);

  const markRulesLoading = useCallback((gameId: string) => {
    dispatch({ type: "rulesPanelLoadStarted", gameId });
  }, []);

  const markRulesLoaded = useCallback((gameId: string, markdown: string) => {
    dispatch({ type: "rulesPanelLoaded", gameId, markdown });
  }, []);

  const markRulesFailed = useCallback((gameId: string) => {
    dispatch({ type: "rulesPanelFailed", gameId });
  }, []);

  const submitStale = useCallback(() => {
    if (!api || !matchId || staleToken === null) {
      return;
    }
    try {
      api.applyAction(matchId, "seat_0", actionTree?.choices[0]?.segment ?? "add-1", staleToken);
    } catch (error: unknown) {
      dispatch({ type: "staleDiagnostic", diagnostic: error as ApiError });
    }
    refresh(api, matchId, effectCursor);
  }, [actionTree, api, effectCursor, matchId, refresh, staleToken]);

  useEffect(() => {
    if (
      !state.autoplay.running ||
      state.setup.playMode !== "bot_vs_bot" ||
      state.pendingOperation !== null ||
      !view ||
      isTerminalView(view) ||
      autoplayInFlightRef.current
    ) {
      return;
    }
    autoplayInFlightRef.current = true;
    const effectsToDrain = effects.filter((entry) => entry.cursor > lastAnimatedCursorRef.current);
    const newestCursor = effectsToDrain.reduce((cursor, entry) => Math.max(cursor, entry.cursor), lastAnimatedCursorRef.current);

    void (async () => {
      try {
        await schedulerRef.current?.enqueueEffects(effectsToDrain);
        lastAnimatedCursorRef.current = newestCursor;
        if (autoplayRunningRef.current) {
          runBotStep();
        }
      } finally {
        autoplayInFlightRef.current = false;
      }
    })();
  }, [effects, runBotStep, state.autoplay.running, state.pendingOperation, state.setup.playMode, view]);

  const textState = useMemo<AppTextState>(
    () => ({
      mode: state.mode === "play" || state.mode === "replay" ? "playing" : state.mode === "setup" ? "ready" : state.mode,
      version,
      matchId,
      view: view ? textView(view, state.selectedGameId) : null,
      choices: actionTree?.choices.map((choice) => choice.segment) ?? [],
      effects: effects.map(summarizeEffect),
      diagnostic,
    }),
    [actionTree, diagnostic, effects, matchId, state.mode, version, view],
  );

  const humanVsBotAdvanceAvailable =
    state.setup.playMode === "human_vs_bot" &&
    Boolean(view && !isTerminalView(view) && view.active_seat && botSeatForMode(state.setup.playMode, view.active_seat));
  const orchestrationActive =
    schedulerActive || autoBotInFlight || state.pendingOperation === "botTurn" || humanVsBotAdvanceAvailable;

  useEffect(() => {
    window.render_game_to_text = () => JSON.stringify(textState);
    window.advanceTime = () => undefined;
  }, [textState]);

  return (
    <AppShell
      version={version}
      reducedMotion={state.reducedMotion}
      rulesPanel={{
        open: state.rulesPanelOpen,
        gameId: state.rulesPanelGameId,
        catalog: state.catalog,
        status: state.rulesPanelStatus,
        markdown: state.rulesPanelMarkdown,
        onClose: closeRules,
        onLoadStarted: markRulesLoading,
        onLoaded: markRulesLoaded,
        onFailed: markRulesFailed,
      }}
    >
      {!matchId ? (
        <>
          <GamePicker
            games={state.catalog}
            selectedGameId={state.selectedGameId}
            onSelect={(gameId) => dispatch({ type: "gameSelected", gameId })}
            onRulesOpen={openRules}
          />
          <MatchSetup
            selectedGame={selectedGame}
            seed={state.setup.seed}
            playMode={state.setup.playMode}
            variantId={state.setup.variantId}
            seatCount={state.setup.seatCount}
            riverLedgerStackMode={state.setup.riverLedgerStackMode}
            riverLedgerStacks={state.setup.riverLedgerStacks}
            canStart={Boolean(api && state.selectedGameId)}
            onSeedChange={(seed) => dispatch({ type: "setupSeedChanged", seed })}
            onPlayModeChange={(playMode) => dispatch({ type: "setupPlayModeChanged", playMode })}
            onVariantChange={(variantId) => dispatch({ type: "setupVariantChanged", variantId })}
            onSeatCountChange={(seatCount) => dispatch({ type: "setupSeatCountChanged", seatCount })}
            onRiverLedgerStackModeChange={(mode) => dispatch({ type: "setupRiverLedgerStackModeChanged", mode })}
            onRiverLedgerStackChange={(index, value) => dispatch({ type: "setupRiverLedgerStackChanged", index, value })}
            onRulesOpen={openRules}
            onStart={start}
          />
        </>
      ) : (
        <>

      <section className="play-surface" aria-label={`${selectedGame?.display_name ?? "Selected game"} play surface`}>
        <SeatFrame
          game={selectedGame}
          view={view}
          viewerMode={state.viewerMode}
          onViewerModeChange={changeSeatFrameViewerMode}
        />

        {state.selectedGameId === "race_to_n" ? (
          <RaceBoard view={isRaceView(view) ? view : null} latestEffect={latestEffect} seatLabels={catalogSeatLabels(selectedGame)} />
        ) : isColumnFourView(view) ? (
          <ColumnFourBoard
            view={view}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatLabels={catalogSeatLabels(selectedGame)}
            onChoice={playChoice}
          />
        ) : isDirectionalFlipView(view) ? (
          <DirectionalFlipBoard
            view={view}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatRoleLabels={seatRoleLabelsForMode(state.setup.playMode)}
            seatLabels={catalogSeatLabels(selectedGame)}
            onChoice={playChoice}
          />
        ) : isDraughtsLiteView(view) ? (
          <DraughtsLiteBoard
            view={view}
            actionTree={actionTree}
            pendingPath={state.pendingActionPath}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPendingPathChange={(path) => dispatch({ type: "pendingActionPathChanged", path })}
            onPendingPathClear={() => dispatch({ type: "pendingActionPathCleared" })}
            onPathSubmit={playPath}
          />
        ) : isHighCardDuelView(view) ? (
          <HighCardDuelBoard
            view={view}
            actionTree={actionTree}
            viewerMode={state.viewerMode}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
            onViewerModeChange={changeViewerMode}
          />
        ) : isTokenBazaarView(view) ? (
          <TokenBazaarBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatRoleLabels={seatRoleLabelsForMode(state.setup.playMode)}
            onChoice={playChoice}
          />
        ) : isSecretDraftView(view) ? (
          <SecretDraftBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
          />
        ) : isPokerLiteView(view) ? (
          <PokerLiteBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
          />
        ) : isRiverLedgerView(view) ? (
          <RiverLedgerBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            lastBotDecision={state.lastBotDecision}
            onChoice={playChoice}
          />
        ) : isPlainTricksView(view) ? (
          <PlainTricksBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
          />
        ) : isBriarCircuitView(view) ? (
          <BriarCircuitBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
            onRestart={start}
          />
        ) : isVowTideView(view) ? (
          <VowTideBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
          />
        ) : isBlackglassPactView(view) ? (
          <BlackglassPactBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
          />
        ) : isMaskedClaimsView(view) ? (
          <MaskedClaimsBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
          />
        ) : isFloodWatchView(view) ? (
          <FloodWatchBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            onPathSubmit={playPath}
          />
        ) : isFrontierControlView(view) ? (
          <FrontierControlBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatLabels={catalogSeatLabels(selectedGame)}
            onPathSubmit={playPath}
          />
        ) : isEventFrontierView(view) ? (
          <EventFrontierBoard
            view={view}
            actionTree={actionTree}
            latestEffect={latestEffect}
            effects={state.effects}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatRoleLabels={seatRoleLabelsForMode(state.setup.playMode)}
            onPathSubmit={playPath}
          />
        ) : isThreeMarksView(view) ? (
          <ThreeMarksBoard
            view={view}
            latestEffect={latestEffect}
            reducedMotion={state.reducedMotion}
            pending={state.pendingOperation !== null}
            seatLabels={catalogSeatLabels(selectedGame)}
            onChoice={playChoice}
          />
        ) : (
          <GenericGameSurface view={view} selectedGameName={selectedGame?.display_name ?? "Selected game"} />
        )}

        {isColumnFourView(view) ||
        isDirectionalFlipView(view) ||
        isDraughtsLiteView(view) ||
        isHighCardDuelView(view) ||
        isTokenBazaarView(view) ||
        isSecretDraftView(view) ||
        isPokerLiteView(view) ||
        isRiverLedgerView(view) ||
        isPlainTricksView(view) ||
        isBriarCircuitView(view) ||
        isVowTideView(view) ||
        isBlackglassPactView(view) ||
        isMaskedClaimsView(view) ||
        isFloodWatchView(view) ||
        isFrontierControlView(view) ||
        isEventFrontierView(view) ? null : (
          <ActionControls
            actionTree={actionTree}
            view={view}
            actorSeat={twoSeatActorSeat(humanActorSeat)}
            pending={state.pendingOperation !== null}
            onChoice={playChoice}
            onRestart={start}
          />
        )}

        <TurnReportPanel gameId={state.selectedGameId} effects={effects} />

        <ModeControls
          playMode={state.setup.playMode}
          view={view}
          gameId={state.selectedGameId}
          gameName={selectedGame?.display_name ?? "selected game"}
          autoplayRunning={state.autoplay.running}
          orchestrationPaused={state.orchestration.paused}
          orchestrationActive={orchestrationActive}
          orchestrationRate={state.orchestration.rate}
          lastBotDecision={state.lastBotDecision}
          pending={state.pendingOperation !== null}
          onRulesOpen={openRules}
          onBotStep={runBotStep}
          onSkip={() => void flushScheduler()}
          onOrchestrationPause={() => dispatch({ type: "orchestrationPaused" })}
          onOrchestrationResume={() => dispatch({ type: "orchestrationResumed" })}
          onOrchestrationRateChange={(rate) => dispatch({ type: "orchestrationRateChanged", rate })}
          onAutoplayStart={() => dispatch({ type: "autoplayStarted" })}
          onAutoplayPause={() => dispatch({ type: "autoplayPaused" })}
        />

        {diagnostic ? (
          <div className="diagnostic" role="status" data-testid="diagnostic">
            <strong>{diagnostic.code}</strong>
            <span>{diagnostic.message}</span>
          </div>
        ) : null}
      </section>

      <EffectLog
        effects={effects}
        reducedMotion={state.reducedMotion}
        override={motion.override}
        onOverrideChange={motion.setOverride}
      />
      <ReplayImportExport canExport={Boolean(matchId)} onExport={exportCurrentReplay} onImport={importReplay} />
      <ReplayViewer
        game={selectedGame}
        replay={state.replay}
        reducedMotion={state.reducedMotion}
        onStep={stepReplay}
        onReset={resetReplay}
      />
        </>
      )}
      <DevPanel
        open={state.devPanelOpen}
        featureReport={state.featureReport}
        selectedGameName={selectedGame?.display_name ?? null}
        matchId={matchId}
        seed={state.setup.seed}
        playMode={state.setup.playMode}
        view={view}
        actionTree={actionTree}
        pendingActionPath={state.pendingActionPath}
        effectCursor={effectCursor}
        effectCount={effects.length}
        pendingOperation={state.pendingOperation}
        replayId={state.replay?.replayId ?? null}
        replayCursor={state.replay?.cursor ?? null}
        diagnostic={diagnostic}
        canSubmitStale={Boolean(api && matchId && staleToken !== null)}
        onToggle={() => dispatch({ type: "devPanelToggled" })}
        onSubmitStale={submitStale}
      />
    </AppShell>
  );
}

declare global {
  interface Window {
    render_game_to_text?: () => string;
    advanceTime?: (ms: number) => void;
  }
}

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Missing #root element");
}

createRoot(rootElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

function humanSeatForMode(playMode: SetupPlayMode, view: PublicView): ViewerSeatId | null {
  if (isTerminalView(view)) {
    return null;
  }
  // Briar Circuit projects an active seat during the pass phase (the next seat that
  // still owes a commitment), so passing is driven by the same active-seat machinery
  // as trick play: the human acts on their own active turn and bot seats are advanced
  // by the auto-bot effect. No pass-specific special case is needed.
  if (playMode === "hotseat") {
    return view.active_seat ?? null;
  }
  if (playMode === "human_vs_bot" && view.active_seat === "seat_0") {
    return "seat_0";
  }
  return null;
}

function botSeatForMode(playMode: SetupPlayMode, seat: ViewerSeatId): boolean {
  if (playMode === "bot_vs_bot") {
    return true;
  }
  return playMode === "human_vs_bot" && seat !== "seat_0";
}

function botSeed(view: PublicView): number {
  return view.freshness_token + (view.active_seat === "seat_0" ? 101 : 211);
}

function seatRoleLabelsForMode(playMode: SetupPlayMode): Partial<Record<SeatId, string>> {
  if (playMode === "human_vs_bot") {
    return { seat_0: "you", seat_1: "bot" };
  }
  if (playMode === "hotseat") {
    return { seat_0: "you", seat_1: "local" };
  }
  return { seat_0: "bot", seat_1: "bot" };
}

function twoSeatActorSeat(seat: ViewerSeatId | null): SeatId | null {
  return seat === "seat_0" || seat === "seat_1" ? seat : null;
}

function selectedVariantForStart(selectedGame: { variants?: Array<{ id: string }> } | null, variantId: string | null): string | undefined {
  if (!selectedGame?.variants || selectedGame.variants.length <= 1) {
    return undefined;
  }
  return selectedGame.variants.some((variant) => variant.id === variantId) ? variantId ?? undefined : selectedGame.variants[0]?.id;
}

function catalogSeatLabels(game: GameCatalogEntry | null): SeatDisplayLabel[] {
  return game?.seat_labels ?? game?.ui?.seat_labels ?? [];
}

function riverLedgerSetupOptions(
  gameId: string,
  stackMode: "standard" | "short_stack" | "custom",
  customStacks: number[],
  seatCount: number | null | undefined,
): { starting_stacks: number[] } | null {
  if (gameId !== "river_ledger" || stackMode === "standard" || typeof seatCount !== "number") {
    return null;
  }
  const startingStacks =
    stackMode === "short_stack"
      ? riverLedgerShortStackPreset(seatCount)
      : Array.from({ length: seatCount }, (_, index) => Math.max(1, Math.trunc(customStacks[index] ?? 24)));
  return { starting_stacks: startingStacks };
}

function parseReplayDocument(documentText: string): ReplayExportDocument | null {
  try {
    return JSON.parse(documentText) as ReplayExportDocument;
  } catch {
    return null;
  }
}

function isRaceView(view: PublicView | null): view is RacePublicView {
  return Boolean(view && "counter" in view);
}

function isThreeMarksView(view: PublicView | null): view is ThreeMarksPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "three_marks");
}

function isColumnFourView(view: PublicView | null): view is ColumnFourPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "column_four");
}

function isDirectionalFlipView(view: PublicView | null): view is DirectionalFlipPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "directional_flip");
}

function isDraughtsLiteView(view: PublicView | null): view is DraughtsLitePublicView {
  return Boolean(view && "game_id" in view && view.game_id === "draughts_lite");
}

function isHighCardDuelView(view: PublicView | null): view is HighCardDuelPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "high_card_duel");
}

function isTokenBazaarView(view: PublicView | null): view is TokenBazaarPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "token_bazaar");
}

function isSecretDraftView(view: PublicView | null): view is SecretDraftPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "secret_draft");
}

function isPokerLiteView(view: PublicView | null): view is PokerLitePublicView {
  return Boolean(view && "game_id" in view && view.game_id === "poker_lite");
}

function isRiverLedgerView(view: PublicView | null): view is RiverLedgerPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "river_ledger");
}

function isPlainTricksView(view: PublicView | null): view is PlainTricksPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "plain_tricks");
}

function isBriarCircuitView(view: PublicView | null): view is BriarCircuitPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "briar_circuit");
}

function isVowTideView(view: PublicView | null): view is VowTidePublicView {
  return Boolean(view && "game_id" in view && view.game_id === "vow_tide");
}

function isBlackglassPactView(view: PublicView | null): view is BlackglassPactPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "blackglass_pact");
}

function isMaskedClaimsView(view: PublicView | null): view is MaskedClaimsPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "masked_claims");
}

function isFloodWatchView(view: PublicView | null): view is FloodWatchPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "flood_watch");
}

function isFrontierControlView(view: PublicView | null): view is FrontierControlPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "frontier_control");
}

function isEventFrontierView(view: PublicView | null): view is EventFrontierPublicView {
  return Boolean(view && "game_id" in view && view.game_id === "event_frontier");
}

function isTerminalView(view: PublicView): boolean {
  if ("winner" in view) {
    return view.winner !== null;
  }
  if (isTokenBazaarView(view)) {
    return view.terminal.terminal;
  }
  if (isSecretDraftView(view)) {
    return view.terminal.terminal;
  }
  if (isPokerLiteView(view)) {
    return view.terminal.terminal;
  }
  if (isRiverLedgerView(view)) {
    return view.terminal.terminal;
  }
  if (isPlainTricksView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  if (isBriarCircuitView(view)) {
    return view.phase === "terminal";
  }
  if (isVowTideView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  if (isBlackglassPactView(view)) {
    return view.phase.kind === "terminal";
  }
  if (isMaskedClaimsView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  if (isFloodWatchView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  if (isFrontierControlView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  if (isEventFrontierView(view)) {
    return view.terminal.kind !== "non_terminal";
  }
  return view.terminal_kind !== "non_terminal";
}

function textView(view: PublicView, fallbackGameId: string): AppTextState["view"] {
  if ("counter" in view) {
    return {
      game_id: fallbackGameId || "race_to_n",
      active_seat: view.active_seat,
      freshness_token: view.freshness_token,
      status: view.winner ? `${view.winner} won` : `${view.counter} / ${view.target}`,
    };
  }
  if (view.game_id === "high_card_duel") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status: `${view.phase} round ${view.round_number}`,
    };
  }
  if (view.game_id === "token_bazaar") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status: view.terminal.terminal
        ? view.terminal.draw
          ? "draw"
          : `${view.terminal.winner} won`
        : `${view.scores.seat_0}-${view.scores.seat_1}, ${view.queue_remaining} queued`,
    };
  }
  if (view.game_id === "secret_draft") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status: view.terminal.terminal
        ? view.terminal.draw
          ? "draw"
          : `${view.terminal.winner} won`
        : `${view.phase} round ${view.round_number}, ${view.scores.seat_0}-${view.scores.seat_1}`,
    };
  }
  if (view.game_id === "poker_lite") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status: view.terminal.terminal
        ? view.terminal.draw
          ? "split"
          : `${view.terminal.winner} won`
        : `${view.phase}, pool ${view.shared_pool}`,
    };
  }
  if (view.game_id === "river_ledger") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status: view.terminal.terminal
        ? view.terminal.winners.length > 1
          ? "split"
          : `${view.terminal.winners[0] ?? "seat_0"} won`
        : `${view.phase}, pot ${view.pot_total}`,
    };
  }
  if (view.game_id === "plain_tricks") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? view.terminal.draw
            ? "split"
            : `${view.terminal.winner} won`
        : `${view.phase}, tricks ${view.total_trick_counts.seat_0}-${view.total_trick_counts.seat_1}`,
    };
  }
  if (view.game_id === "briar_circuit") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? view.viewer_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.phase === "terminal"
          ? "terminal"
          : `${view.phase}, scores ${view.cumulative_scores.seat_0}-${view.cumulative_scores.seat_1}-${view.cumulative_scores.seat_2}-${view.cumulative_scores.seat_3}`,
    };
  }
  if (view.game_id === "vow_tide") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? view.terminal.winners.length > 1
            ? "shared win"
            : `${view.terminal.winners[0] ?? "seat_0"} won`
          : `${view.phase}, hand ${view.hand_index + 1}, size ${view.hand_size}`,
    };
  }
  if (view.game_id === "blackglass_pact") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? view.viewer_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.phase.kind === "terminal"
          ? `${view.phase.winning_team} won`
          : `${view.phase.kind}, scores ${view.team_scores.team_0}-${view.team_scores.team_1}`,
    };
  }
  if (view.game_id === "masked_claims") {
    return {
      game_id: view.game_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? view.terminal.draw
            ? "draw"
            : `${view.terminal.winner} won`
          : `${view.phase}, scores ${view.scores.seat_0}-${view.scores.seat_1}`,
    };
  }
  if (view.game_id === "flood_watch") {
    return {
      game_id: view.game_id,
      variant_id: view.variant_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? view.terminal.summary.public_summary
          : `turn ${view.turn_number}, budget ${view.phase.kind === "action" ? view.phase.budget_remaining : 0}, undrawn ${view.undrawn_count}`,
    };
  }
  if (view.game_id === "frontier_control") {
    return {
      game_id: view.game_id,
      variant_id: view.variant_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? view.terminal.summary
          : `round ${view.round_number}, budget ${view.phase.kind === "action" ? view.phase.budget_remaining : 0}, scores ${view.scores.garrison}-${view.scores.prospectors}`,
    };
  }
  if (view.game_id === "event_frontier") {
    return {
      game_id: view.game_id,
      variant_id: view.variant_id,
      active_seat: view.active_seat ?? "seat_0",
      freshness_token: view.freshness_token,
      status:
        view.terminal.kind !== "non_terminal"
          ? `${view.terminal.winner} won by ${view.terminal.victory_type}`
          : `epoch ${view.epoch}, card ${view.current_card?.label ?? "none"}, scores ${view.scores.charter}-${view.scores.freeholders}`,
    };
  }
  return {
    game_id: view.game_id,
    active_seat: view.active_seat ?? "seat_0",
    freshness_token: view.freshness_token,
    status: view.status_label,
  };
}

function effectiveViewerMode(
  api: RulepathApi,
  matchId: string,
  playMode: SetupPlayMode,
  currentViewerMode: ViewerMode,
): ViewerMode {
  if (playMode === "bot_vs_bot") {
    return { kind: "observer" };
  }
  if (playMode === "hotseat") {
    const observerView = api.getView(matchId, { kind: "observer" });
    return observerView.active_seat ? { kind: "seat", seat: observerView.active_seat } : currentViewerMode;
  }
  return { kind: "seat", seat: "seat_0" };
}

function activeViewerSeatIds(view: PublicView | null, game: GameCatalogEntry | null): Set<string> {
  if (view && "active_seat_labels" in view && Array.isArray(view.active_seat_labels)) {
    return new Set(view.active_seat_labels.map((label) => label.seat));
  }
  const catalogLabels = game?.seat_labels ?? game?.ui?.seat_labels ?? [];
  if (catalogLabels.length > 0) {
    return new Set(catalogLabels.map((label) => label.seat));
  }
  const viewerModes = game?.viewer_modes?.filter((mode) => mode !== "observer") ?? [];
  if (viewerModes.length > 0) {
    return new Set(viewerModes);
  }
  if (!view) return new Set();
  if ("active_seats" in view && Array.isArray(view.active_seats)) {
    return new Set(view.active_seats);
  }
  return view.active_seat ? new Set([view.active_seat]) : new Set();
}

function isViewerSeatId(value: string): value is ViewerSeatId {
  return ["seat_0", "seat_1", "seat_2", "seat_3", "seat_4", "seat_5", "seat_6"].includes(value);
}

function GenericGameSurface({
  view,
  selectedGameName,
}: {
  view: PublicView | null;
  selectedGameName: string;
}) {
  const boardShape = view && "board_rows" in view ? `${view.board_rows} x ${view.board_columns}` : "Rust view";
  const status = view
    ? "status_label" in view
      ? view.status_label
      : isSecretDraftView(view)
        ? view.terminal.terminal
          ? view.terminal.draw
            ? "Draw"
            : `${view.terminal.winner} won`
          : `${view.phase} round ${view.round_number}`
      : isTokenBazaarView(view)
        ? view.terminal.terminal
          ? view.terminal.draw
            ? "Draw"
            : `${view.terminal.winner} won`
          : `${view.active_seat} to move`
        : `${view.active_seat} to move`
    : "Ready";

  return (
    <section className="race-board generic-board" aria-label={`${selectedGameName} match`}>
      <div className="scoreboard">
        <div>
          <span>Game</span>
          <strong>{selectedGameName}</strong>
        </div>
        <div>
          <span>Status</span>
          <strong data-testid="turn">{status}</strong>
        </div>
        <div>
          <span>Token</span>
          <strong>{view?.freshness_token ?? "--"}</strong>
        </div>
      </div>

      <div className="board-status" role="status">
        <span data-testid="three-marks-started">{view ? `${boardShape} match started from Rust` : "Waiting for Rust view"}</span>
      </div>
    </section>
  );
}

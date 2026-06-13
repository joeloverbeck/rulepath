import type { PublicView } from "../wasm/client";

export type SettleIssue = {
  code: "missing_board" | "lingering_ghost" | "running_animation";
  detail: string;
};

export type SettleAssertionResult = {
  ok: boolean;
  issues: SettleIssue[];
};

type SettleRoot = ParentNode & {
  getAnimations?: () => Animation[];
};

export function createDevSettleAssertion(getView: () => PublicView | null, root: SettleRoot = document): () => void {
  return () => {
    if (!devSettleAssertionEnabled()) {
      return;
    }
    const result = assertSettledView(root, getView());
    if (!result.ok) {
      console.warn("[rulepath:settle-assertion]", result.issues);
    }
  };
}

export function assertSettledView(root: SettleRoot, view: PublicView | null): SettleAssertionResult {
  const issues: SettleIssue[] = [];

  if (!view) {
    return { ok: true, issues };
  }

  const gameId = gameIdForView(view);
  const boardSelector = `[data-testid="${boardTestId(gameId)}"]`;
  if (!root.querySelector(boardSelector)) {
    issues.push({
      code: "missing_board",
      detail: `settled view for ${gameId} is missing ${boardSelector}`,
    });
  }

  if (root.querySelector(".animation-ghost")) {
    issues.push({
      code: "lingering_ghost",
      detail: "animation ghost remained after scheduler settle",
    });
  }

  const running = root.getAnimations?.().filter((animation) => animation.playState === "running") ?? [];
  if (running.length > 0) {
    issues.push({
      code: "running_animation",
      detail: `${running.length} animation(s) still running after scheduler settle`,
    });
  }

  return { ok: issues.length === 0, issues };
}

function devSettleAssertionEnabled(): boolean {
  return Boolean(
    import.meta.env.DEV &&
      typeof globalThis.localStorage !== "undefined" &&
      globalThis.localStorage.getItem("rulepath.devSettleAssertion") === "on",
  );
}

function boardTestId(gameId: string): string {
  return `${gameId.replaceAll("_", "-")}-board`;
}

function gameIdForView(view: PublicView): string {
  return "game_id" in view ? view.game_id : "race_to_n";
}

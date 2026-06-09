import type { ReactNode } from "react";
import { RulesPanel } from "./RulesPanel";
import type { RulesPanelStatus } from "../state/shellReducer";
import type { GameCatalogEntry } from "../wasm/client";

type AppShellProps = {
  version: string;
  reducedMotion: boolean;
  rulesPanel: {
    open: boolean;
    gameId: string | null;
    catalog: GameCatalogEntry[];
    status: RulesPanelStatus;
    markdown: string | null;
    onClose: () => void;
    onLoadStarted: (gameId: string) => void;
    onLoaded: (gameId: string, markdown: string) => void;
    onFailed: (gameId: string) => void;
  };
  children: ReactNode;
};

export function AppShell({ version, reducedMotion, rulesPanel, children }: AppShellProps) {
  return (
    <main className={reducedMotion ? "shell reduced-motion" : "shell"}>
      <header className="topbar">
        <div>
          <p className="eyebrow">Rulepath</p>
          <h1>Rulepath</h1>
        </div>
        <p className="wasm-status" data-testid="wasm-status">
          {version}
        </p>
      </header>
      {children}
      <RulesPanel
        open={rulesPanel.open}
        gameId={rulesPanel.gameId}
        catalog={rulesPanel.catalog}
        status={rulesPanel.status}
        markdown={rulesPanel.markdown}
        onClose={rulesPanel.onClose}
        onLoadStarted={rulesPanel.onLoadStarted}
        onLoaded={rulesPanel.onLoaded}
        onFailed={rulesPanel.onFailed}
      />
    </main>
  );
}

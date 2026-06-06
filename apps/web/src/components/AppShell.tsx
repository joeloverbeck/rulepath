import type { ReactNode } from "react";

type AppShellProps = {
  version: string;
  reducedMotion: boolean;
  children: ReactNode;
};

export function AppShell({ version, reducedMotion, children }: AppShellProps) {
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
    </main>
  );
}

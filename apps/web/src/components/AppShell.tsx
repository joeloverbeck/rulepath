import type { ReactNode } from "react";

type AppShellProps = {
  version: string;
  children: ReactNode;
};

export function AppShell({ version, children }: AppShellProps) {
  return (
    <main className="shell">
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

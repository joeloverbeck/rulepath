import { useMemo, useState } from "react";
import type { ApiError, ReplayExportDocument } from "../wasm/client";

type ReplayImportExportProps = {
  canExport: boolean;
  onExport: () => ReplayExportDocument;
  onImport: (documentText: string) => void;
};

const MAX_COMMAND_SUMMARY_CHARS = 512 * 1024;

export function ReplayImportExport({ canExport, onExport, onImport }: ReplayImportExportProps) {
  const [documentText, setDocumentText] = useState("");
  const [diagnostic, setDiagnostic] = useState<ApiError | null>(null);
  const commandSummary = useMemo(() => replayCommandSummary(documentText), [documentText]);

  const exportReplay = () => {
    setDiagnostic(null);
    const document = onExport();
    setDocumentText(JSON.stringify(document, null, 2));
  };

  const importReplay = () => {
    setDiagnostic(null);
    try {
      onImport(normalizeJsonDocument(documentText));
    } catch (error: unknown) {
      setDiagnostic(error as ApiError);
    }
  };

  return (
    <section className="replay-io" aria-labelledby="replay-io-heading">
      <div className="region-heading">
        <p className="eyebrow">Replay</p>
        <h2 id="replay-io-heading">Import / export</h2>
      </div>
      <div className="replay-actions">
        <button type="button" onClick={exportReplay} disabled={!canExport}>
          Export Current Run
        </button>
        <button type="button" onClick={importReplay} disabled={documentText.trim().length === 0}>
          Import Replay
        </button>
      </div>
      <label className="replay-document-field">
        <span>Replay document</span>
        <textarea
          value={documentText}
          onChange={(event) => setDocumentText(event.currentTarget.value)}
          spellCheck={false}
        />
      </label>
      {commandSummary.length > 0 ? (
        <ol className="replay-command-summary" aria-label="Replay command paths">
          {commandSummary.map((command) => (
            <li key={command.index}>
              <span>{command.index + 1}</span>
              <strong>{command.actor}</strong>
              <code>{command.path}</code>
            </li>
          ))}
        </ol>
      ) : null}
      {diagnostic ? (
        <div className="diagnostic" role="status">
          <strong>{diagnostic.code}</strong>
          <span>{diagnostic.message}</span>
        </div>
      ) : null}
    </section>
  );
}

function normalizeJsonDocument(documentText: string): string {
  return JSON.stringify(JSON.parse(documentText));
}

function replayCommandSummary(documentText: string): { index: number; actor: string; path: string }[] {
  if (!documentText.trim()) {
    return [];
  }
  if (documentText.length > MAX_COMMAND_SUMMARY_CHARS) {
    return [];
  }
  try {
    const document = JSON.parse(documentText) as ReplayExportDocument;
    if (!("commands" in document)) {
      return [];
    }
    return (document.commands ?? []).map((command) => ({
      index: command.index,
      actor: command.actor_seat,
      path: command.action_path.join(" > "),
    }));
  } catch {
    return [];
  }
}

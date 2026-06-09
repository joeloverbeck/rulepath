import { useEffect, useMemo, useRef } from "react";
import type { ReactNode } from "react";
import type { RulesPanelStatus } from "../state/shellReducer";
import type { GameCatalogEntry } from "../wasm/client";

type RulesPanelProps = {
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

type Heading = {
  id: string;
  level: number;
  text: string;
};

const GAME_ID_RE = /^[a-z0-9_]+$/;

export function RulesPanel({
  open,
  gameId,
  catalog,
  status,
  markdown,
  onClose,
  onLoadStarted,
  onLoaded,
  onFailed,
}: RulesPanelProps) {
  const closeButtonRef = useRef<HTMLButtonElement | null>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);
  const validGame = useMemo(
    () => (gameId && GAME_ID_RE.test(gameId) ? catalog.find((game) => game.game_id === gameId) ?? null : null),
    [catalog, gameId],
  );
  const article = useMemo(() => renderRulesMarkdown(markdown ?? ""), [markdown]);

  useEffect(() => {
    if (!open) return;
    previousFocusRef.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    requestAnimationFrame(() => closeButtonRef.current?.focus());
  }, [open, gameId]);

  useEffect(() => {
    if (!open || !gameId) return;
    if (!validGame) {
      onFailed(gameId);
      return;
    }

    const controller = new AbortController();
    const assetUrl = `${import.meta.env.BASE_URL}rules/${validGame.game_id}.md`;
    onLoadStarted(validGame.game_id);

    fetch(assetUrl, { signal: controller.signal })
      .then((response) => {
        if (!response.ok) {
          throw new Error("rules_unavailable");
        }
        return response.text();
      })
      .then((text) => onLoaded(validGame.game_id, text))
      .catch((error: unknown) => {
        if (error instanceof DOMException && error.name === "AbortError") return;
        onFailed(validGame.game_id);
      });

    return () => controller.abort();
  }, [gameId, onFailed, onLoadStarted, onLoaded, open, validGame]);

  useEffect(() => {
    if (!open) return;
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        onClose();
      }
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [onClose, open]);

  useEffect(() => {
    if (open) return;
    previousFocusRef.current?.focus();
  }, [open]);

  if (!open) return null;

  const titleId = "rules-panel-title";
  const title = validGame ? `${validGame.display_name} Rules` : "Rules";

  return (
    <div className="rules-panel-backdrop" role="presentation">
      <aside className="rules-panel" role="dialog" aria-modal="true" aria-labelledby={titleId}>
        <header className="rules-panel-header">
          <div>
            <p className="eyebrow">How to Play</p>
            <h2 id={titleId}>{title}</h2>
          </div>
          <button type="button" className="rules-panel-close" onClick={onClose} ref={closeButtonRef}>
            Close
          </button>
        </header>

        {status === "loading" ? (
          <div className="rules-panel-status" role="status">
            Loading rules...
          </div>
        ) : status === "error" ? (
          <div className="rules-panel-status error" role="status">
            Rules are unavailable for this game.
          </div>
        ) : (
          <div className="rules-panel-content">
            {article.headings.length > 1 ? (
              <nav className="rules-toc" aria-label="Rules sections">
                <span>Sections</span>
                <ol>
                  {article.headings
                    .filter((heading) => heading.level === 2)
                    .map((heading) => (
                      <li key={heading.id}>
                        <a href={`#${heading.id}`}>{heading.text}</a>
                      </li>
                    ))}
                </ol>
              </nav>
            ) : null}
            <article className="rules-article">{article.nodes}</article>
          </div>
        )}
      </aside>
    </div>
  );
}

export function renderRulesMarkdown(markdown: string): { headings: Heading[]; nodes: ReactNode[] } {
  const lines = markdown.replace(/\r\n/g, "\n").split("\n");
  const headings: Heading[] = [];
  const nodes: ReactNode[] = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];
    if (!line.trim()) {
      i += 1;
      continue;
    }

    const heading = /^(#{1,4})\s+(.+)$/.exec(line);
    if (heading) {
      const level = heading[1].length;
      const text = stripInlineMarkdown(heading[2].trim());
      const id = uniqueSlug(text, headings);
      headings.push({ id, level, text });
      const HeadingTag = `h${Math.min(level + 1, 4)}` as "h2" | "h3" | "h4";
      nodes.push(
        <HeadingTag id={id} key={`${id}-${nodes.length}`}>
          {renderInline(heading[2].trim())}
        </HeadingTag>,
      );
      i += 1;
      continue;
    }

    if (isTableStart(lines, i)) {
      const rows: string[][] = [];
      const header = splitTableRow(lines[i]);
      i += 2;
      while (i < lines.length && /^\|.*\|$/.test(lines[i].trim())) {
        rows.push(splitTableRow(lines[i]));
        i += 1;
      }
      nodes.push(
        <table key={`table-${nodes.length}`}>
          <thead>
            <tr>
              {header.map((cell, index) => (
                <th key={index}>{renderInline(cell)}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex}>{renderInline(cell)}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>,
      );
      continue;
    }

    if (/^-\s+/.test(line)) {
      const items: string[] = [];
      while (i < lines.length && /^-\s+/.test(lines[i])) {
        items.push(lines[i].replace(/^-\s+/, ""));
        i += 1;
      }
      nodes.push(
        <ul key={`ul-${nodes.length}`}>
          {items.map((item, index) => (
            <li key={index}>{renderInline(item)}</li>
          ))}
        </ul>,
      );
      continue;
    }

    if (/^\d+\.\s+/.test(line)) {
      const items: string[] = [];
      while (i < lines.length && /^\d+\.\s+/.test(lines[i])) {
        items.push(lines[i].replace(/^\d+\.\s+/, ""));
        i += 1;
      }
      nodes.push(
        <ol key={`ol-${nodes.length}`}>
          {items.map((item, index) => (
            <li key={index}>{renderInline(item)}</li>
          ))}
        </ol>,
      );
      continue;
    }

    const paragraph: string[] = [];
    while (
      i < lines.length &&
      lines[i].trim() &&
      !/^(#{1,4})\s+/.test(lines[i]) &&
      !/^-\s+/.test(lines[i]) &&
      !/^\d+\.\s+/.test(lines[i]) &&
      !isTableStart(lines, i)
    ) {
      paragraph.push(lines[i].trim().replace(/\s{2}$/, ""));
      i += 1;
    }
    nodes.push(<p key={`p-${nodes.length}`}>{renderInline(paragraph.join(" "))}</p>);
  }

  return { headings, nodes };
}

function renderInline(text: string): ReactNode[] {
  const nodes: ReactNode[] = [];
  const tokenRe = /(`[^`]+`|\*\*[^*]+\*\*|_[^_]+_)/g;
  let lastIndex = 0;
  let match: RegExpExecArray | null;
  while ((match = tokenRe.exec(text))) {
    if (match.index > lastIndex) {
      nodes.push(text.slice(lastIndex, match.index));
    }
    const token = match[0];
    const key = `${match.index}-${token}`;
    if (token.startsWith("`")) {
      nodes.push(<code key={key}>{token.slice(1, -1)}</code>);
    } else if (token.startsWith("**")) {
      nodes.push(<strong key={key}>{token.slice(2, -2)}</strong>);
    } else {
      nodes.push(<em key={key}>{token.slice(1, -1)}</em>);
    }
    lastIndex = match.index + token.length;
  }
  if (lastIndex < text.length) {
    nodes.push(text.slice(lastIndex));
  }
  return nodes;
}

function stripInlineMarkdown(text: string): string {
  return text.replace(/[`*_]/g, "");
}

function uniqueSlug(text: string, headings: Heading[]): string {
  const base = text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
  const used = new Set(headings.map((heading) => heading.id));
  if (!used.has(base)) return base || "section";
  let suffix = 2;
  while (used.has(`${base}-${suffix}`)) suffix += 1;
  return `${base}-${suffix}`;
}

function isTableStart(lines: string[], index: number): boolean {
  return /^\|.*\|$/.test(lines[index]?.trim() ?? "") && /^\|\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?$/.test(lines[index + 1]?.trim() ?? "");
}

function splitTableRow(line: string): string[] {
  return line
    .trim()
    .replace(/^\|/, "")
    .replace(/\|$/, "")
    .split("|")
    .map((cell) => cell.trim());
}

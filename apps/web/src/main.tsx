import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./styles.css";

type WasmExports = {
  memory: WebAssembly.Memory;
  rulepath_placeholder_version_ptr: () => number;
  rulepath_placeholder_version_len: () => number;
};

async function loadPlaceholderVersion(): Promise<string> {
  const response = await fetch("/wasm_api.wasm");
  if (!response.ok) {
    throw new Error(`Unable to load wasm-api artifact: ${response.status}`);
  }

  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const exports = instance.exports as WasmExports;
  const ptr = exports.rulepath_placeholder_version_ptr();
  const len = exports.rulepath_placeholder_version_len();
  const view = new Uint8Array(exports.memory.buffer, ptr, len);

  return new TextDecoder().decode(view);
}

function App() {
  const [version, setVersion] = useState("Loading wasm-api...");

  useEffect(() => {
    let cancelled = false;

    loadPlaceholderVersion()
      .then((loadedVersion) => {
        if (!cancelled) {
          setVersion(loadedVersion);
        }
      })
      .catch((error: unknown) => {
        if (!cancelled) {
          setVersion(error instanceof Error ? error.message : "Unable to load wasm-api artifact");
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main className="shell">
      <section className="status-panel" aria-label="WASM status">
        <p className="eyebrow">Rulepath</p>
        <h1>Gate 0 Shell</h1>
        <p className="wasm-status">{version}</p>
      </section>
    </main>
  );
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

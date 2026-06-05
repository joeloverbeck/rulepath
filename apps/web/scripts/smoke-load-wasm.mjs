import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const artifactPath = join(__dirname, "..", "public", "wasm_api.wasm");
const bytes = await readFile(artifactPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const exports = instance.exports;

const ptr = exports.rulepath_placeholder_version_ptr();
const len = exports.rulepath_placeholder_version_len();
const view = new Uint8Array(exports.memory.buffer, ptr, len);
const version = new TextDecoder().decode(view);

if (version !== "rulepath-wasm-api/0.1.0") {
  throw new Error(`Unexpected wasm-api version string: ${version}`);
}

console.log(version);

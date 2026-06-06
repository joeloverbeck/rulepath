import { createReadStream } from "node:fs";
import { readFile } from "node:fs/promises";
import http from "node:http";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const distDir = join(__dirname, "..", "dist");
const mountPath = "/rulepath/";

const server = http.createServer(async (request, response) => {
  if (!request.url?.startsWith(mountPath)) {
    response.writeHead(404);
    response.end("not found");
    return;
  }

  const relativeUrl = request.url.slice(mountPath.length).split("?")[0] || "index.html";
  const safePath = normalize(relativeUrl).replace(/^(\.\.[/\\])+/, "");
  const filePath = join(distDir, safePath);
  const contentType = contentTypeFor(filePath);

  try {
    await readFile(filePath);
    response.writeHead(200, { "Content-Type": contentType });
    createReadStream(filePath).pipe(response);
  } catch {
    response.writeHead(200, { "Content-Type": "text/html" });
    createReadStream(join(distDir, "index.html")).pipe(response);
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));

try {
  const { port } = server.address();
  const baseUrl = `http://127.0.0.1:${port}${mountPath}`;
  const index = await (await fetch(baseUrl)).text();
  assert(index.includes("./assets/"), "built index uses relative asset paths");

  const wasmResponse = await fetch(new URL("wasm_api.wasm", baseUrl));
  assert(wasmResponse.ok, "served dist exposes wasm artifact under nested mount");
  const bytes = await wasmResponse.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const wasm = instance.exports;
  const decoder = new TextDecoder();
  const encoder = new TextEncoder();

  const version = read(wasm, decoder, wasm.rulepath_placeholder_version_ptr(), wasm.rulepath_placeholder_version_len());
  assert(version === "rulepath-wasm-api/0.1.0", "served wasm artifact loads");

  const created = invoke(wasm, encoder, decoder, (args) => wasm.rulepath_new_match(args[0].ptr, args[0].len, 1n), [
    "race_to_n",
  ]);
  const tree = invoke(
    wasm,
    encoder,
    decoder,
    (args) => wasm.rulepath_get_action_tree(args[0].ptr, args[0].len, args[1].ptr, args[1].len),
    [created.match_id, "seat_0"],
  );
  const afterHuman = invoke(
    wasm,
    encoder,
    decoder,
    (args) =>
      wasm.rulepath_apply_action(
        args[0].ptr,
        args[0].len,
        args[1].ptr,
        args[1].len,
        args[2].ptr,
        args[2].len,
        0n,
      ),
    [created.match_id, "seat_0", tree.choices[0].segment],
  );
  if (!afterHuman.view.winner) {
    invoke(
      wasm,
      encoder,
      decoder,
      (args) => wasm.rulepath_run_bot_turn(args[0].ptr, args[0].len, args[1].ptr, args[1].len, 44n),
      [created.match_id, afterHuman.view.active_seat],
    );
  }

  console.log(JSON.stringify({ base: mountPath, version, match_id: created.match_id }));
} finally {
  await new Promise((resolve) => server.close(resolve));
}

function invoke(wasm, encoder, decoder, call, values) {
  const args = values.map((value) => write(wasm, encoder, value));
  try {
    const status = call(args);
    const parsed = JSON.parse(read(wasm, decoder, wasm.rulepath_last_output_ptr(), wasm.rulepath_last_output_len()));
    if (status !== 0) {
      throw new Error(parsed.message);
    }
    return parsed;
  } finally {
    for (const arg of args) {
      wasm.rulepath_dealloc(arg.ptr, arg.len);
    }
  }
}

function write(wasm, encoder, value) {
  const bytes = encoder.encode(value);
  const ptr = wasm.rulepath_alloc(bytes.length);
  new Uint8Array(wasm.memory.buffer, ptr, bytes.length).set(bytes);
  return { ptr, len: bytes.length };
}

function read(wasm, decoder, ptr, len) {
  return decoder.decode(new Uint8Array(wasm.memory.buffer, ptr, len));
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function contentTypeFor(path) {
  switch (extname(path)) {
    case ".html":
      return "text/html";
    case ".js":
      return "text/javascript";
    case ".css":
      return "text/css";
    case ".wasm":
      return "application/wasm";
    default:
      return "application/octet-stream";
  }
}

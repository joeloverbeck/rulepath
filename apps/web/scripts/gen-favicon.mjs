// Generates public/favicon.ico (16x16 + 32x32) by rasterizing the same
// geometry as public/favicon.svg. Zero dependencies; rerun after changing
// the icon design so the two assets never drift.
//
//   node scripts/gen-favicon.mjs

import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

// Geometry in the SVG's 64-unit space (keep in sync with favicon.svg).
const TILE_RADIUS = 14;
const PATH_POINTS = [
  [17, 47],
  [17, 31],
  [33, 31],
  [33, 17],
  [47, 17],
];
const PATH_HALF_WIDTH = 3.5;
const DOT_CENTER = [47, 17];
const DOT_RADIUS = 6.5;

const TILE_COLOR = [0x2b, 0x80, 0x68];
const PATH_COLOR = [0xf9, 0xfa, 0xf7];
const DOT_COLOR = [0xf5, 0xc4, 0x6b];

function insideRoundedTile(x, y) {
  if (x < 0 || x > 64 || y < 0 || y > 64) return false;
  const r = TILE_RADIUS;
  const cx = x < r ? r : x > 64 - r ? 64 - r : x;
  const cy = y < r ? r : y > 64 - r ? 64 - r : y;
  if (cx === x || cy === y) return true;
  return (x - cx) ** 2 + (y - cy) ** 2 <= r * r;
}

function distToSegment(x, y, [ax, ay], [bx, by]) {
  const dx = bx - ax;
  const dy = by - ay;
  const lenSq = dx * dx + dy * dy;
  const t = Math.max(0, Math.min(1, ((x - ax) * dx + (y - ay) * dy) / lenSq));
  return Math.hypot(x - (ax + t * dx), y - (ay + t * dy));
}

function onPath(x, y) {
  for (let i = 0; i < PATH_POINTS.length - 1; i++) {
    if (distToSegment(x, y, PATH_POINTS[i], PATH_POINTS[i + 1]) <= PATH_HALF_WIDTH) {
      return true;
    }
  }
  return false;
}

function sampleColor(x, y) {
  if (!insideRoundedTile(x, y)) return null;
  if (Math.hypot(x - DOT_CENTER[0], y - DOT_CENTER[1]) <= DOT_RADIUS) return DOT_COLOR;
  if (onPath(x, y)) return PATH_COLOR;
  return TILE_COLOR;
}

// Render one size as RGBA via 4x4 supersampling of the 64-unit space.
function render(size) {
  const pixels = Buffer.alloc(size * size * 4);
  const SS = 4;
  const step = 64 / size / SS;
  for (let py = 0; py < size; py++) {
    for (let px = 0; px < size; px++) {
      let r = 0;
      let g = 0;
      let b = 0;
      let a = 0;
      for (let sy = 0; sy < SS; sy++) {
        for (let sx = 0; sx < SS; sx++) {
          const c = sampleColor(
            (px * SS + sx + 0.5) * step,
            (py * SS + sy + 0.5) * step,
          );
          if (c) {
            r += c[0];
            g += c[1];
            b += c[2];
            a += 255;
          }
        }
      }
      const n = SS * SS;
      const i = (py * size + px) * 4;
      pixels[i] = Math.round(r / n);
      pixels[i + 1] = Math.round(g / n);
      pixels[i + 2] = Math.round(b / n);
      pixels[i + 3] = Math.round(a / n);
    }
  }
  return pixels;
}

// One ICO image entry: BITMAPINFOHEADER + bottom-up BGRA rows + AND mask.
function icoImage(size, rgba) {
  const header = Buffer.alloc(40);
  header.writeUInt32LE(40, 0); // header size
  header.writeInt32LE(size, 4); // width
  header.writeInt32LE(size * 2, 8); // height (XOR + AND masks)
  header.writeUInt16LE(1, 12); // planes
  header.writeUInt16LE(32, 14); // bits per pixel
  const xor = Buffer.alloc(size * size * 4);
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const src = (y * size + x) * 4;
      const dst = ((size - 1 - y) * size + x) * 4;
      xor[dst] = rgba[src + 2];
      xor[dst + 1] = rgba[src + 1];
      xor[dst + 2] = rgba[src];
      xor[dst + 3] = rgba[src + 3];
    }
  }
  const andMask = Buffer.alloc((((size + 31) >> 5) << 2) * size);
  return Buffer.concat([header, xor, andMask]);
}

const sizes = [16, 32];
const images = sizes.map((s) => icoImage(s, render(s)));
const fileHeader = Buffer.alloc(6);
fileHeader.writeUInt16LE(1, 2); // type: icon
fileHeader.writeUInt16LE(sizes.length, 4);
const entries = [];
let offset = 6 + sizes.length * 16;
for (let i = 0; i < sizes.length; i++) {
  const entry = Buffer.alloc(16);
  entry.writeUInt8(sizes[i] === 256 ? 0 : sizes[i], 0); // width
  entry.writeUInt8(sizes[i] === 256 ? 0 : sizes[i], 1); // height
  entry.writeUInt16LE(1, 4); // planes
  entry.writeUInt16LE(32, 6); // bits per pixel
  entry.writeUInt32LE(images[i].length, 8);
  entry.writeUInt32LE(offset, 12);
  offset += images[i].length;
  entries.push(entry);
}

const out = join(dirname(fileURLToPath(import.meta.url)), "..", "public", "favicon.ico");
writeFileSync(out, Buffer.concat([fileHeader, ...entries, ...images]));
console.log(`wrote ${out}`);

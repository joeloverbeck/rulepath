// Shared Puppeteer launcher for the web e2e / smoke scripts.
//
// On a cold CI runner Chrome occasionally fails to report its DevTools
// WebSocket endpoint within the default launch window ("Timed out after 30000
// ms while waiting for the WS endpoint URL to appear in stdout"). That is an
// environmental flake, not a product defect, so the launch is wrapped in a
// bounded retry with a longer timeout and the standard container-stability
// flags. Every e2e/smoke script launches through here, so the hardening lives
// in one place instead of in ~19 duplicated launch blocks.
import { access } from "node:fs/promises";
import { setTimeout as sleep } from "node:timers/promises";
import puppeteer from "puppeteer";

const defaultExecutablePath =
  process.env.PUPPETEER_EXECUTABLE_PATH || "/usr/bin/google-chrome";

const MAX_ATTEMPTS = 3;
const RETRY_DELAY_MS = 1000;
const LAUNCH_TIMEOUT_MS = 60000;

export async function launchBrowser(executablePath = defaultExecutablePath) {
  await access(executablePath);

  let lastError;
  for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt += 1) {
    try {
      return await puppeteer.launch({
        executablePath,
        headless: "new",
        args: ["--no-sandbox", "--disable-setuid-sandbox", "--disable-dev-shm-usage"],
        timeout: LAUNCH_TIMEOUT_MS,
      });
    } catch (error) {
      lastError = error;
      if (attempt < MAX_ATTEMPTS) {
        console.warn(
          `puppeteer.launch attempt ${attempt}/${MAX_ATTEMPTS} failed: ${error.message}; ` +
            `retrying in ${RETRY_DELAY_MS}ms`,
        );
        await sleep(RETRY_DELAY_MS);
      }
    }
  }

  throw lastError;
}

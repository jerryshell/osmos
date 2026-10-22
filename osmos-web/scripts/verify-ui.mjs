import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "dist");
const port = 4173;
const seconds = Number(process.env.VERIFY_SECONDS || 60);
const types = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".wasm": "application/wasm",
  ".svg": "image/svg+xml",
  ".json": "application/json"
};

const server = createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", `http://127.0.0.1:${port}`);
  let file = path.join(root, decodeURIComponent(url.pathname));
  if (file.endsWith("/")) file = path.join(file, "index.html");
  if (!file.startsWith(root)) {
    response.writeHead(403);
    response.end();
    return;
  }

  try {
    const body = await readFile(file);
    response.writeHead(200, {
      "content-type": types[path.extname(file)] ?? "application/octet-stream",
      "cache-control": "no-store"
    });
    response.end(body);
  } catch {
    const body = await readFile(path.join(root, "index.html"));
    response.writeHead(200, { "content-type": types[".html"] });
    response.end(body);
  }
});

await new Promise((resolve) => server.listen(port, "127.0.0.1", resolve));

const browser = await chromium.launch({
  headless: true,
  executablePath: process.env.CHROME_PATH || undefined,
  args: ["--no-sandbox", "--disable-dev-shm-usage"]
});

if (process.env.VERIFY_INTERACTIONS === "1") {
  const errors = [];
  const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  await page.goto(`http://127.0.0.1:${port}/?seed=123`, { waitUntil: "networkidle" });
  await page.waitForTimeout(300);
  await page.getByRole("button", { name: "Pause" }).click();
  const paused = await page.getByRole("button", { name: "Resume" }).getAttribute("aria-pressed");
  await page.getByRole("button", { name: "Reset" }).click();
  const resetUrl = page.url();

  const reducedErrors = [];
  const reduced = await browser.newPage({ viewport: { width: 390, height: 844 } });
  reduced.on("pageerror", (error) => reducedErrors.push(String(error)));
  await reduced.emulateMedia({ reducedMotion: "reduce" });
  await reduced.goto(`http://127.0.0.1:${port}/?seed=123`, { waitUntil: "networkidle" });
  await reduced.waitForTimeout(300);
  const reducedPaused = await reduced.getByRole("button", { name: "Resume" }).getAttribute("aria-pressed");

  console.log(JSON.stringify({ case: "interactions", paused, resetUrl, reducedPaused, errors, reducedErrors }));
  await page.close();
  await reduced.close();
  await browser.close();
  await new Promise((resolve) => server.close(resolve));
  process.exit(0);
}

const cases = [
  { name: "desktop-1x", viewport: { width: 1280, height: 800 }, speed: 1 },
  { name: "desktop-8x", viewport: { width: 1280, height: 800 }, speed: 8 },
  { name: "mobile-1x", viewport: { width: 390, height: 844 }, speed: 1 },
  { name: "mobile-8x", viewport: { width: 390, height: 844 }, speed: 8 }
];

for (const item of cases) {
  const page = await browser.newPage({ viewport: item.viewport, deviceScaleFactor: 1 });
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  await page.goto(`http://127.0.0.1:${port}/?seed=42`, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: `${item.speed}×` }).click();
  await page.waitForTimeout(300);

  const smallButtons = await page.evaluate(() =>
    [...document.querySelectorAll("button")]
      .map((button) => {
        const rect = button.getBoundingClientRect();
        return { text: button.textContent?.trim(), width: rect.width, height: rect.height };
      })
      .filter((button) => button.width < 44 || button.height < 44)
  );

  const result = await page.evaluate(
    async ({ durationMs }) => {
      const read = () => {
        const text = document.querySelector("#hud .meta")?.textContent ?? "";
        const match = text.match(/TICK\s+(\d+)\s+·\s+(\d+)\s+ALIVE\s+·\s+(\d+)\s+PELLETS/);
        return match
          ? { tick: Number(match[1]), alive: Number(match[2]), pellets: Number(match[3]) }
          : { tick: 0, alive: 0, pellets: 0 };
      };

      const start = read();
      const frames = [];
      let previous = performance.now();
      const started = previous;

      await new Promise((resolve) => {
        const tick = (now) => {
          if (now - started >= durationMs) {
            resolve();
            return;
          }
          frames.push(now - previous);
          previous = now;
          requestAnimationFrame(tick);
        };
        requestAnimationFrame(tick);
      });

      const sorted = [...frames].sort((a, b) => a - b);
      const percentile = (value) => sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * value))] ?? 0;
      const end = read();

      return {
        durationSeconds: durationMs / 1000,
        frames: frames.length,
        fps: frames.length / (durationMs / 1000),
        frameP50: percentile(0.5),
        frameP95: percentile(0.95),
        start,
        end
      };
    },
    { durationMs: seconds * 1000 }
  );

  console.log(
    JSON.stringify({
      case: item.name,
      ...result,
      smallButtons,
      errors
    })
  );

  await page.close();
}

await browser.close();
await new Promise((resolve) => server.close(resolve));

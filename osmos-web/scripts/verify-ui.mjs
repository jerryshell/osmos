import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, firefox, webkit } from "playwright";

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

const browserName = process.env.VERIFY_BROWSER || "chromium";
const browserType = { chromium, firefox, webkit }[browserName];
if (!browserType) {
  throw new Error(`unknown VERIFY_BROWSER: ${browserName}`);
}
const browser = await browserType.launch({
  headless: true,
  ...(browserName === "chromium"
    ? {
        executablePath: process.env.CHROME_PATH || undefined,
        args: ["--no-sandbox", "--disable-dev-shm-usage"]
      }
    : {})
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
  await page.getByRole("button", { name: "Orbital" }).click();
  const orbitalUrl = page.url();
  const orbitalPressed = await page.getByRole("button", { name: "Orbital" }).getAttribute("aria-pressed");
  await page.getByRole("button", { name: "Repulsor" }).click();
  const repulsorUrl = page.url();
  const repulsorPressed = await page.getByRole("button", { name: "Repulsor" }).getAttribute("aria-pressed");
  await page.getByRole("button", { name: "Switch to Chinese" }).click();
  const zhLang = await page.evaluate(() => document.documentElement.lang);
  const zhBest = await page.locator("#hud .eyebrow").textContent();
  const zhUrl = page.url();
  await page.getByRole("button", { name: "切换到英文" }).click();
  const enLang = await page.evaluate(() => document.documentElement.lang);
  const enBest = await page.locator("#hud .eyebrow").textContent();
  const enUrl = page.url();

  const reducedErrors = [];
  const reduced = await browser.newPage({ viewport: { width: 390, height: 844 } });
  reduced.on("pageerror", (error) => reducedErrors.push(String(error)));
  await reduced.emulateMedia({ reducedMotion: "reduce" });
  await reduced.goto(`http://127.0.0.1:${port}/?seed=123`, { waitUntil: "networkidle" });
  await reduced.waitForTimeout(300);
  const reducedPaused = await reduced.getByRole("button", { name: "Resume" }).getAttribute("aria-pressed");

  console.log(
    JSON.stringify({
      case: "interactions",
      paused,
      resetUrl,
      orbitalUrl,
      orbitalPressed,
      repulsorUrl,
      repulsorPressed,
      zhLang,
      zhBest,
      zhUrl,
      enLang,
      enBest,
      enUrl,
      reducedPaused,
      errors,
      reducedErrors
    })
  );
  await page.close();
  await reduced.close();
  await browser.close();
  await new Promise((resolve) => server.close(resolve));
  process.exit(0);
}

const cases = [
  { name: "desktop-1x", viewport: { width: 1280, height: 800 }, speed: 1 },
  { name: "desktop-4x", viewport: { width: 1280, height: 800 }, speed: 4 },
  { name: "desktop-8x", viewport: { width: 1280, height: 800 }, speed: 8 },
  { name: "mobile-1x", viewport: { width: 390, height: 844 }, speed: 1 },
  { name: "mobile-4x", viewport: { width: 390, height: 844 }, speed: 4 },
  { name: "mobile-8x", viewport: { width: 390, height: 844 }, speed: 8 },
  {
    name: "mobile-orbital-4x",
    viewport: { width: 390, height: 844 },
    speed: 4,
    scenario: "orbital"
  },
  {
    name: "mobile-repulsor-4x",
    viewport: { width: 390, height: 844 },
    speed: 4,
    scenario: "repulsor"
  }
];

const only = process.env.VERIFY_CASE;
const selectedCases = only ? cases.filter((item) => item.name === only) : cases;
if (selectedCases.length === 0) {
  throw new Error(`unknown VERIFY_CASE: ${only}`);
}

for (const item of selectedCases) {
  const page = await browser.newPage({ viewport: item.viewport, deviceScaleFactor: 1 });
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });

  await page.addInitScript(() => {
    window.__longTasks = [];
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) window.__longTasks.push(entry.duration);
    }).observe({ entryTypes: ["longtask"] });
  });
  const query = item.scenario ? `?seed=42&scenario=${item.scenario}` : "?seed=42";
  await page.goto(`http://127.0.0.1:${port}/${query}`, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: `${item.speed}×` }).click();
  await page.waitForTimeout(300);
  const memoryStart = await page.evaluate(() => {
    const memory = performance.memory;
    return memory ? memory.usedJSHeapSize : null;
  });
  await page.evaluate(() => {
    window.__osmosPerf?.reset();
    window.__longTasks = [];
  });

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
        const match = text.match(
          /GEN\s+(\d+)\s+·\s+TICK\s+(\d+)\s+·\s+(\d+)\s+ALIVE\s+·\s+(\d+)\s+PELLETS/
        );
        return match
          ? {
              generation: Number(match[1]),
              tick: Number(match[2]),
              alive: Number(match[3]),
              pellets: Number(match[4]),
              best: Number(document.querySelector("#hud .best")?.textContent ?? 0)
            }
          : { generation: 0, tick: 0, alive: 0, pellets: 0, best: 0 };
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

  const perf = await page.evaluate(() => window.__osmosPerf?.summary() ?? null);
  const champion = await page.evaluate(
    () => document.querySelector("#app")?.getAttribute("data-champion") ?? null
  );
  const memoryEnd = await page.evaluate(() => {
    const memory = performance.memory;
    return memory ? memory.usedJSHeapSize : null;
  });
  console.log(
    JSON.stringify({
      case: item.name,
      browser: browserName,
      champion,
      memoryStart,
      memoryEnd,
      ...result,
      perf,
      smallButtons,
      errors
    })
  );

  await page.close();
}

await browser.close();
await new Promise((resolve) => server.close(resolve));

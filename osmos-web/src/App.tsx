import { createEffect, createSignal, onCleanup, onMount } from "solid-js";
import * as osmos from "../osmos-wasm";
import championData from "./champion.json";
import {
  localeTag,
  messages,
  persistLocale,
  readLocale,
  scenarioNames,
  type Locale,
  type MessageKey
} from "./i18n";

type Stats = {
  generation: number;
  tick: number;
  alive: number;
  pellets: number;
  ejecta: number;
  bestMass: number;
  totalMass: number;
};

type Detail = {
  id: number;
  mass: number;
  sensor: Float32Array;
  thrustX: number;
  thrustY: number;
  absorbedPellets: number;
  absorbedMotes: number;
  ejectedMass: number;
};

type Snapshot = {
  tick: number;
  moteIds: Uint32Array;
  moteState: Float32Array;
  moteFlags: Uint8Array;
  pelletState: Float32Array;
  ejectaState: Float32Array;
  wellState: Float32Array;
  stats: Stats;
};

const MOTE_STRIDE = 7;
const PELLET_STRIDE = 4;
const EJECTA_STRIDE = 5;
const WELL_STRIDE = 4;
const MAX_STEPS_PER_FRAME = 16;
const CHAMPION = Float32Array.from(championData.genome);
const CHAMPION_LOADED = CHAMPION.length === osmos.genomeLength();

const createEngine = (seed: number, scenario: number): osmos.Engine => {
  if (CHAMPION_LOADED) {
    return osmos.newEngineWithChampion(seed, scenario, CHAMPION);
  }
  return new osmos.Engine(seed, scenario);
};

type PerfSummary = {
  stepP50: number;
  stepP95: number;
  stepMax: number;
  snapshotP50: number;
  snapshotP95: number;
  snapshotMax: number;
  frameMax: number;
  longTasks: number;
};

declare global {
  interface Window {
    __osmosPerf?: {
      reset: () => void;
      summary: () => PerfSummary;
    };
  }
}

const readSeed = (): number => {
  const value = new URLSearchParams(window.location.search).get("seed");
  const parsed = value ? Number.parseInt(value, 10) : Number.NaN;
  return Number.isFinite(parsed) ? parsed >>> 0 : (Date.now() >>> 0);
};

const writeState = (seed: number, scenario: number, locale: Locale) => {
  const url = new URL(window.location.href);
  url.searchParams.set("seed", String(seed));
  const name =
    scenario === 1 ? "sentient" : scenario === 2 ? "orbital" : scenario === 3 ? "repulsor" : "ambient";
  url.searchParams.set("scenario", name);
  url.searchParams.set("lang", locale);
  window.history.replaceState(null, "", url);
};

const readScenario = (): number => {
  const value = new URLSearchParams(window.location.search).get("scenario");
  if (value === "sentient" || value === "1") return 1;
  if (value === "orbital" || value === "2") return 2;
  if (value === "repulsor" || value === "3") return 3;
  return 0;
};

// === 状态 ===
const App = () => {
  const [locale, setLocale] = createSignal<Locale>(readLocale());
  const t = (key: MessageKey) => messages[locale()][key];
  const [scenario, setScenario] = createSignal(0);
  const [paused, setPaused] = createSignal(false);
  const [speed, setSpeed] = createSignal(1);
  const [generation, setGeneration] = createSignal(0);
  const [tick, setTick] = createSignal(0);
  const [alive, setAlive] = createSignal(0);
  const [pellets, setPellets] = createSignal(0);
  const [snapshotEjecta, setSnapshotEjecta] = createSignal(0);
  const [bestMass, setBestMass] = createSignal(0);
  const [seed, setSeed] = createSignal(0);
  const [selectedId, setSelectedId] = createSignal<number | null>(null);
  const [detail, setDetail] = createSignal<Detail | null>(null);
  const [error, setError] = createSignal("");

  createEffect(() => {
    document.documentElement.lang = localeTag(locale());
    document.title = t("documentTitle");
  });

  const toggleLocale = () => {
    const next: Locale = locale() === "en" ? "zh" : "en";
    setLocale(next);
    persistLocale(next);
    if (engine) writeState(seed(), scenario(), next);
  };

  let canvas: HTMLCanvasElement | undefined;
  let engine: osmos.Engine | undefined;
  let ctx: CanvasRenderingContext2D | undefined;
  let frame = 0;

  const resize = () => {
    if (!canvas) return;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    canvas.width = Math.max(1, Math.round(window.innerWidth * dpr));
    canvas.height = Math.max(1, Math.round(window.innerHeight * dpr));
    canvas.style.width = `${window.innerWidth}px`;
    canvas.style.height = `${window.innerHeight}px`;
    ctx = canvas.getContext("2d") ?? undefined;
    ctx?.setTransform(dpr, 0, 0, dpr, 0, 0);
  };

  const fieldLayout = () => {
    const width = window.innerWidth;
    const height = window.innerHeight;
    const mobile = width < 640;
    const padding = mobile ? 14 : 28;
    const size = Math.min(width, height) - padding * 2;
    const left = (width - size) / 2;
    const top = mobile ? 132 : (height - size) / 2;
    return { width, height, size, left, top };
  };

  // === 渲染 ===
  const draw = (snapshot: Snapshot) => {
    if (!ctx) return;

    const { width, height, size, left, top } = fieldLayout();

    ctx.clearRect(0, 0, width, height);

    const field = ctx.createRadialGradient(
      left + size * 0.5,
      top + size * 0.42,
      size * 0.04,
      left + size * 0.5,
      top + size * 0.5,
      size * 0.72
    );
    field.addColorStop(0, "rgba(18, 48, 55, 0.68)");
    field.addColorStop(0.58, "rgba(8, 22, 25, 0.72)");
    field.addColorStop(1, "rgba(4, 11, 13, 0.9)");
    ctx.fillStyle = field;
    ctx.fillRect(left, top, size, size);

    ctx.strokeStyle = "rgba(49, 214, 196, 0.14)";
    ctx.lineWidth = 1;
    ctx.setLineDash([3, 7]);
    ctx.strokeRect(left + 0.5, top + 0.5, size - 1, size - 1);
    ctx.setLineDash([]);

    const wells = snapshot.wellState ?? new Float32Array(0);
    for (let index = 0; index < wells.length; index += WELL_STRIDE) {
      const x = left + wells[index] * size;
      const y = top + (1 - wells[index + 1]) * size;
      const radius = Math.max(7, wells[index + 2] * size);
      const kind = wells[index + 3];
      const glow = ctx.createRadialGradient(x, y, 0, x, y, radius * 3.2);
      if (kind > 0) {
        glow.addColorStop(0, "rgba(49, 214, 196, 0.22)");
        glow.addColorStop(0.45, "rgba(49, 214, 196, 0.07)");
        glow.addColorStop(1, "rgba(49, 214, 196, 0)");
      } else {
        glow.addColorStop(0, "rgba(255, 200, 87, 0.2)");
        glow.addColorStop(0.45, "rgba(255, 200, 87, 0.06)");
        glow.addColorStop(1, "rgba(255, 200, 87, 0)");
      }
      ctx.fillStyle = glow;
      ctx.beginPath();
      ctx.arc(x, y, radius * 3.2, 0, Math.PI * 2);
      ctx.fill();
      ctx.beginPath();
      ctx.strokeStyle = kind > 0 ? "rgba(49, 214, 196, 0.62)" : "rgba(255, 200, 87, 0.72)";
      ctx.lineWidth = 1.5;
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.stroke();
      ctx.beginPath();
      ctx.fillStyle = kind > 0 ? "#31d6c4" : "#ffc857";
      ctx.arc(x, y, 2.5, 0, Math.PI * 2);
      ctx.fill();
    }

    const pellets = snapshot.pelletState;
    ctx.beginPath();
    for (let index = 0; index < pellets.length; index += PELLET_STRIDE) {
      const x = left + pellets[index] * size;
      const y = top + (1 - pellets[index + 1]) * size;
      const radius = Math.max(1.1, pellets[index + 3] * size);
      ctx.moveTo(x + radius, y);
      ctx.arc(x, y, radius, 0, Math.PI * 2);
    }
    ctx.fillStyle = "rgba(208, 235, 228, 0.42)";
    ctx.fill();

    const ejecta = snapshot.ejectaState;
    for (let index = 0; index < ejecta.length; index += EJECTA_STRIDE) {
      const x = left + ejecta[index] * size;
      const y = top + (1 - ejecta[index + 1]) * size;
      const radius = Math.max(1, ejecta[index + 3] * size);
      const hue = ejecta[index + 4] * 360;
      ctx.beginPath();
      ctx.fillStyle = `hsla(${hue}, 64%, 72%, 0.55)`;
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.fill();
    }

    const motes = snapshot.moteState;
    for (let index = 0; index < motes.length; index += MOTE_STRIDE) {
      const x = left + motes[index] * size;
      const y = top + (1 - motes[index + 1]) * size;
      const mass = motes[index + 4];
      const radius = Math.max(2, motes[index + 5] * size);
      const hue = motes[index + 6] * 360;

      ctx.beginPath();
      ctx.fillStyle = `hsl(${hue}, 64%, 72%)`;
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.fill();

      if (mass >= snapshot.stats.bestMass - 1e-5) {
        ctx.beginPath();
        ctx.strokeStyle = "rgba(255, 255, 255, 0.62)";
        ctx.lineWidth = 1;
        ctx.arc(x, y, radius + 4, 0, Math.PI * 2);
        ctx.stroke();
      }

      if (snapshot.moteIds[index / MOTE_STRIDE] === selectedId()) {
        ctx.beginPath();
        ctx.strokeStyle = "#ffffff";
        ctx.lineWidth = 2;
        ctx.arc(x, y, radius + 7, 0, Math.PI * 2);
        ctx.stroke();

        const current = detail();
        if (current && current.id === selectedId()) {
          const length = Math.max(12, radius * 2.4);
          ctx.beginPath();
          ctx.strokeStyle = "#31d6c4";
          ctx.lineWidth = 2;
          ctx.moveTo(x, y);
          ctx.lineTo(x + current.thrustX * length, y - current.thrustY * length);
          ctx.stroke();
        }
      }
    }
  };

  const applyStats = (snapshot: Snapshot) => {
    setGeneration(snapshot.stats.generation);
    setTick(snapshot.stats.tick);
    setAlive(snapshot.stats.alive);
    setPellets(snapshot.stats.pellets);
    setSnapshotEjecta(snapshot.stats.ejecta);
    setBestMass(snapshot.stats.bestMass);

    const selected = selectedId();
    if (selected !== null && engine) {
      const current = engine.inspect(selected) as Detail | null;
      setDetail(current);
      if (!current) setSelectedId(null);
    }
  };

  const reset = () => {
    if (!engine) return;
    const next = (Math.random() * 0xffff_ffff) >>> 0;
    engine.reset(next);
    setSeed(next);
    setPaused(false);
    resetStats();
    setSelectedId(null);
    setDetail(null);
    writeState(next, scenario(), locale());
  };

  const resetStats = () => {
    setGeneration(0);
    setTick(0);
    setAlive(0);
    setPellets(0);
    setSnapshotEjecta(0);
    setBestMass(0);
  };

  const chooseScenario = (next: number) => {
    if (next === scenario()) return;
    setScenario(next);
    engine = createEngine(seed(), next);
    setSpeed(1);
    setPaused(false);
    resetStats();
    setSelectedId(null);
    setDetail(null);
    writeState(seed(), next, locale());
  };

  const sensorBars = (offset: number) =>
    Array.from({ length: 16 }, (_, index) => {
      const value = detail()?.sensor[offset + index] ?? 0;
      return <i style={{ height: `${Math.max(2, Math.min(100, Math.abs(value) * 100))}%` }} />;
    });

  const signalBar = (index: number) => {
    const value = Math.max(0, Math.min(1, detail()?.sensor[index] ?? 0));
    return (
      <div class="sensor-bars single">
        <i style={{ height: `${Math.max(2, value * 100)}%` }} />
      </div>
    );
  };

  // === 主循环 ===
  onMount(() => {
    if (!canvas) return;

    const initialSeed = readSeed();
    const initialScenario = readScenario();
    setSeed(initialSeed);
    setScenario(initialScenario);
    writeState(initialSeed, initialScenario, locale());
    resize();

    try {
      engine = createEngine(initialSeed, initialScenario);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : t("errorMessage"));
      return;
    }

    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      setPaused(true);
    }

    let last = performance.now();
    let lastStats = 0;
    let accumulator = 0;
    const stepTimes: number[] = [];
    const snapshotTimes: number[] = [];
    const frameTimes: number[] = [];

    window.__osmosPerf = {
      reset: () => {
        stepTimes.length = 0;
        snapshotTimes.length = 0;
        frameTimes.length = 0;
      },
      summary: () => {
        const percentile = (values: number[], value: number) => {
          if (values.length === 0) return 0;
          const sorted = [...values].sort((a, b) => a - b);
          return sorted[Math.min(sorted.length - 1, Math.floor(sorted.length * value))];
        };
        const high = (values: number[]) =>
          values.length === 0 ? 0 : values.reduce((max, value) => Math.max(max, value), 0);
        return {
          stepP50: percentile(stepTimes, 0.5),
          stepP95: percentile(stepTimes, 0.95),
          stepMax: high(stepTimes),
          snapshotP50: percentile(snapshotTimes, 0.5),
          snapshotP95: percentile(snapshotTimes, 0.95),
          snapshotMax: high(snapshotTimes),
          frameMax: high(frameTimes),
          longTasks: (window as Window & { __longTasks?: number[] }).__longTasks?.length ?? 0
        };
      }
    };

    const render = (now: number) => {
      const frameStart = performance.now();
      const elapsed = Math.min((now - last) / 1000, 0.1);
      last = now;

      let snapshotStart = 0;
      if (!paused()) {
        accumulator += elapsed * 60 * speed();
        const wanted = Math.floor(accumulator);
        const steps = Math.min(wanted, MAX_STEPS_PER_FRAME);
        accumulator -= wanted;
        const stepStart = performance.now();
        if (steps > 0) {
          engine?.step(steps);
        }
        snapshotStart = performance.now();
        stepTimes.push(snapshotStart - stepStart);
      } else {
        accumulator = 0;
      }

      const snapshot = engine?.snapshot();
      const snapshotEnd = performance.now();
      if (snapshotStart > 0) {
        snapshotTimes.push(snapshotEnd - snapshotStart);
      }
      if (snapshot) {
        draw(snapshot);
        if (now - lastStats >= 250) {
          applyStats(snapshot);
          lastStats = now;
        }
      }

      frameTimes.push(performance.now() - frameStart);
      frame = requestAnimationFrame(render);
    };

    const onResize = () => {
      resize();
      const snapshot = engine?.snapshot();
      if (snapshot) draw(snapshot);
    };

    const onPointerDown = (event: PointerEvent) => {
      if (!canvas || !engine) return;
      const snapshot = engine.snapshot() as Snapshot;
      const { size, left, top } = fieldLayout();
      const rect = canvas.getBoundingClientRect();
      const x = (event.clientX - rect.left - left) / size;
      const y = 1 - (event.clientY - rect.top - top) / size;

      let bestId: number | null = null;
      let bestDistance = Number.POSITIVE_INFINITY;
      for (let index = 0; index < snapshot.moteState.length; index += MOTE_STRIDE) {
        const mx = snapshot.moteState[index];
        const my = snapshot.moteState[index + 1];
        const radius = snapshot.moteState[index + 5];
        const distance = Math.hypot(mx - x, my - y);
        if (distance <= radius + 0.02 && distance < bestDistance) {
          bestDistance = distance;
          bestId = snapshot.moteIds[index / MOTE_STRIDE];
        }
      }

      if (bestId === null) {
        setSelectedId(null);
        setDetail(null);
        return;
      }
      setSelectedId(bestId);
      setDetail(engine.inspect(bestId) as Detail | null);
    };

    const onKeyDown = (event: KeyboardEvent) => {
      if (event.code === "Space") {
        event.preventDefault();
        setPaused(!paused());
      } else if (event.key === "Escape") {
        setSelectedId(null);
        setDetail(null);
      } else if (event.key === "r" || event.key === "R") {
        reset();
      } else if (event.key === "ArrowUp") {
        event.preventDefault();
        setSpeed((value) => Math.min(8, value * 2));
      } else if (event.key === "ArrowDown") {
        event.preventDefault();
        setSpeed((value) => Math.max(1, value / 2));
      }
    };

    window.addEventListener("resize", onResize);
    window.addEventListener("keydown", onKeyDown);
    canvas.addEventListener("pointerdown", onPointerDown);
    frame = requestAnimationFrame(render);

    onCleanup(() => {
      cancelAnimationFrame(frame);
      window.removeEventListener("resize", onResize);
      window.removeEventListener("keydown", onKeyDown);
      canvas?.removeEventListener("pointerdown", onPointerDown);
    });
  });

  // === 界面 ===
  return (
    <div id="app" data-champion={CHAMPION_LOADED ? "true" : "false"}>
      <header id="topbar">
        <h1 class="brand">
          OSMOS <span>FIELD</span>
        </h1>
        <p class="tagline">{t("tagline")}</p>
        <div class="scenarios" role="group" aria-label={t("scenarioGroup")}>
          <button
            type="button"
            aria-pressed={scenario() === 0}
            onClick={() => chooseScenario(0)}
          >
            {scenarioNames[locale()][0]}
          </button>
          <button
            type="button"
            aria-pressed={scenario() === 1}
            onClick={() => chooseScenario(1)}
          >
            {scenarioNames[locale()][1]}
          </button>
          <button
            type="button"
            aria-pressed={scenario() === 2}
            onClick={() => chooseScenario(2)}
          >
            {scenarioNames[locale()][2]}
          </button>
          <button
            type="button"
            aria-pressed={scenario() === 3}
            onClick={() => chooseScenario(3)}
          >
            {scenarioNames[locale()][3]}
          </button>
        </div>
        <span class="seed">{t("seed")} {seed().toString(16).toUpperCase().padStart(8, "0")}</span>
      </header>

      <main id="stage">
        <canvas
          ref={(element) => {
            canvas = element;
          }}
          role="img"
          aria-label={t("canvasLabel")}
        />
      </main>

      {selectedId() !== null && detail() && (
        <aside id="inspector" aria-label={t("selectedMote")}>
          <button
            class="inspector-close"
            type="button"
            aria-label={t("closeInspector")}
            onClick={() => {
              setSelectedId(null);
              setDetail(null);
            }}
          >
            ×
          </button>
          <p class="inspector-eyebrow">{t("lineage")} {String(selectedId()! % 12).padStart(2, "0")}</p>
          <h2 class="inspector-title">{t("mote")} #{selectedId()}</h2>
          <p class="inspector-sub">
            {t("massLong")} {detail()!.mass.toFixed(2)} · {t("absorbed")} {detail()!.absorbedPellets} {t("pellets")} · {detail()!.absorbedMotes} {t("motes")}
          </p>

          <div class="inspector-metrics">
            <div><span>{t("massShort")}</span><strong>{detail()!.mass.toFixed(2)}</strong></div>
            <div><span>{t("ejected")}</span><strong>{detail()!.ejectedMass.toFixed(2)}</strong></div>
          </div>

          <p class="inspector-section">{t("sensoryRing")}</p>
          <div class="sensor-row"><span>{t("massShort")}</span><div class="sensor-bars">{sensorBars(0)}</div></div>
          <div class="sensor-row"><span>{t("proximity")}</span><div class="sensor-bars">{sensorBars(16)}</div></div>
          <div class="sensor-row"><span>{t("radial")}</span><div class="sensor-bars radial">{sensorBars(32)}</div></div>
          <div class="sensor-row"><span>{t("threat")}</span>{signalBar(51)}</div>
          <div class="sensor-row"><span>{t("food")}</span>{signalBar(53)}</div>

          <p class="inspector-section">{t("controlOutput")}</p>
          <div class="inspector-output">
            <div><span>{t("thrustX")}</span><strong>{detail()!.thrustX >= 0 ? "+" : ""}{detail()!.thrustX.toFixed(2)}</strong></div>
            <div><span>{t("thrustY")}</span><strong>{detail()!.thrustY >= 0 ? "+" : ""}{detail()!.thrustY.toFixed(2)}</strong></div>
          </div>
        </aside>
      )}

      <section id="hud">
        <span class="eyebrow">{t("bestMass")}</span>
        <strong class="best">{bestMass().toFixed(2)}</strong>
        <span class="meta">
          {locale() === "zh"
            ? `${t("gen")} ${generation()} 代 · ${t("tick")} ${tick()} · ${t("alive")} ${alive()} · ${t("pellets")} ${pellets()} · ${t("ejecta")} ${snapshotEjecta()}`
            : `${t("gen")} ${generation()} · ${t("tick")} ${tick()} · ${alive()} ${t("alive")} · ${pellets()} ${t("pellets").toUpperCase()} · ${snapshotEjecta()} ${t("ejecta")}`}
        </span>
      </section>

      <p id="caption">
        {scenario() === 2
          ? t("captionOrbital")
          : scenario() === 3
            ? t("captionRepulsor")
            : t("captionMass")}
      </p>

      <footer id="controls">
        <button
          type="button"
          aria-pressed={paused()}
          onClick={() => setPaused(!paused())}
        >
          {paused() ? t("resume") : t("pause")}
        </button>

        <div class="speed-group" role="group" aria-label={t("speedGroup")}>
          <button
            type="button"
            aria-pressed={speed() === 1}
            onClick={() => setSpeed(1)}
          >
            1×
          </button>
          <button
            type="button"
            aria-pressed={speed() === 2}
            onClick={() => setSpeed(2)}
          >
            2×
          </button>
          <button
            type="button"
            aria-pressed={speed() === 4}
            onClick={() => setSpeed(4)}
          >
            4×
          </button>
          <button
            type="button"
            aria-pressed={speed() === 8}
            onClick={() => setSpeed(8)}
          >
            8×
          </button>
        </div>

        <button type="button" onClick={reset}>
          {t("reset")}
        </button>

        <button
          class="locale-switch"
          type="button"
          aria-label={locale() === "en" ? t("switchToChinese") : t("switchToEnglish")}
          title={locale() === "en" ? t("switchToChinese") : t("switchToEnglish")}
          onClick={toggleLocale}
        >
          {locale() === "en" ? t("languageChinese") : t("languageEnglish")}
        </button>
      </footer>

      {error() && (
        <div class="error" role="alert">
          <strong>{t("errorTitle")}</strong>
          <span>{error()}</span>
          <button type="button" onClick={() => window.location.reload()}>
            {t("retry")}
          </button>
        </div>
      )}
    </div>
  );
};

export default App;

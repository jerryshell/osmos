import { createSignal, onCleanup, onMount } from "solid-js";
import * as osmos from "../osmos-wasm";

type Stats = {
  tick: number;
  alive: number;
  pellets: number;
  bestMass: number;
  totalMass: number;
};

type Snapshot = {
  tick: number;
  moteIds: Uint32Array;
  moteState: Float32Array;
  moteRole: Uint8Array;
  moteFlags: Uint8Array;
  pelletState: Float32Array;
  stats: Stats;
};

const MOTE_STRIDE = 7;
const PELLET_STRIDE = 4;
const MAX_STEPS_PER_FRAME = 16;

const readSeed = (): number => {
  const value = new URLSearchParams(window.location.search).get("seed");
  const parsed = value ? Number.parseInt(value, 10) : Number.NaN;
  return Number.isFinite(parsed) ? parsed >>> 0 : (Date.now() >>> 0);
};

const writeSeed = (seed: number) => {
  const url = new URL(window.location.href);
  url.searchParams.set("seed", String(seed));
  window.history.replaceState(null, "", url);
};

const App = () => {
  const [paused, setPaused] = createSignal(false);
  const [speed, setSpeed] = createSignal(1);
  const [tick, setTick] = createSignal(0);
  const [alive, setAlive] = createSignal(0);
  const [pellets, setPellets] = createSignal(0);
  const [bestMass, setBestMass] = createSignal(0);
  const [seed, setSeed] = createSignal(0);
  const [error, setError] = createSignal("");

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

  const draw = (snapshot: Snapshot) => {
    if (!ctx) return;

    const width = window.innerWidth;
    const height = window.innerHeight;
    const mobile = width < 640;
    const padding = mobile ? 14 : 28;
    const size = Math.min(width, height) - padding * 2;
    const left = (width - size) / 2;
    const top = mobile ? 132 : (height - size) / 2;

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

    ctx.strokeStyle = "rgba(49, 214, 196, 0.24)";
    ctx.lineWidth = 1;
    ctx.strokeRect(left + 0.5, top + 0.5, size - 1, size - 1);

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

    const motes = snapshot.moteState;
    const roles = snapshot.moteRole;
    for (let index = 0; index < motes.length; index += MOTE_STRIDE) {
      const x = left + motes[index] * size;
      const y = top + (1 - motes[index + 1]) * size;
      const mass = motes[index + 4];
      const radius = Math.max(2, motes[index + 5] * size);
      const hue = motes[index + 6] * 360;
      const role = roles[index / MOTE_STRIDE] ?? 0;

      ctx.beginPath();
      ctx.fillStyle = `hsl(${hue}, 64%, 72%)`;
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      ctx.fill();

      if (role === 1 || role === 2) {
        ctx.beginPath();
        ctx.strokeStyle = role === 1 ? "#ff7369" : "#ffc857";
        ctx.lineWidth = 1.5;
        ctx.arc(x, y, radius + 3, 0, Math.PI * 2);
        ctx.stroke();
      }

      if (mass >= snapshot.stats.bestMass - 1e-5) {
        ctx.beginPath();
        ctx.strokeStyle = "rgba(255, 255, 255, 0.62)";
        ctx.lineWidth = 1;
        ctx.arc(x, y, radius + 4, 0, Math.PI * 2);
        ctx.stroke();
      }
    }
  };

  const applyStats = (snapshot: Snapshot) => {
    setTick(snapshot.stats.tick);
    setAlive(snapshot.stats.alive);
    setPellets(snapshot.stats.pellets);
    setBestMass(snapshot.stats.bestMass);
  };

  const reset = () => {
    if (!engine) return;
    const next = (Math.random() * 0xffff_ffff) >>> 0;
    engine.reset(next);
    setSeed(next);
    setPaused(false);
    setTick(0);
    setAlive(0);
    setPellets(0);
    setBestMass(0);
    writeSeed(next);
  };

  onMount(() => {
    if (!canvas) return;

    const initialSeed = readSeed();
    setSeed(initialSeed);
    writeSeed(initialSeed);
    resize();

    try {
      engine = new osmos.Engine(initialSeed);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : "The simulation engine failed to load.");
      return;
    }

    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      setPaused(true);
    }

    let last = performance.now();
    let lastStats = 0;
    let accumulator = 0;

    const render = (now: number) => {
      const elapsed = Math.min((now - last) / 1000, 0.1);
      last = now;

      if (!paused()) {
        accumulator += elapsed * 60 * speed();
        const wanted = Math.floor(accumulator);
        const steps = Math.min(wanted, MAX_STEPS_PER_FRAME);
        accumulator -= wanted;
        if (steps > 0) {
          engine?.step(steps);
        }
      } else {
        accumulator = 0;
      }

      const snapshot = engine?.snapshot();
      if (snapshot) {
        draw(snapshot);
        if (now - lastStats >= 250) {
          applyStats(snapshot);
          lastStats = now;
        }
      }

      frame = requestAnimationFrame(render);
    };

    const onResize = () => {
      resize();
      const snapshot = engine?.snapshot();
      if (snapshot) draw(snapshot);
    };

    window.addEventListener("resize", onResize);
    frame = requestAnimationFrame(render);

    onCleanup(() => {
      cancelAnimationFrame(frame);
      window.removeEventListener("resize", onResize);
    });
  });

  return (
    <div id="app">
      <header id="topbar">
        <h1 class="brand">
          OSMOS <span>FIELD</span>
        </h1>
        <p class="tagline">mass, momentum, absorption</p>
        <span class="seed">SEED {seed().toString(16).toUpperCase().padStart(8, "0")}</span>
      </header>

      <main id="stage">
        <canvas
          ref={(element) => {
            canvas = element;
          }}
          role="img"
          aria-label="A simulation of motes absorbing smaller motes and pellets. Each circle is a mote, and larger circles have more mass."
        />
      </main>

      <section id="hud">
        <span class="eyebrow">BEST MASS</span>
        <strong class="best">{bestMass().toFixed(2)}</strong>
        <span class="meta">
          TICK {tick()} · {alive()} ALIVE · {pellets()} PELLETS
        </span>
      </section>

      <p id="caption">MASS IS THE ONLY RESOURCE. LARGER CIRCLES ABSORB SMALLER ONES.</p>

      <footer id="controls">
        <button
          type="button"
          aria-pressed={paused()}
          onClick={() => setPaused(!paused())}
        >
          {paused() ? "Resume" : "Pause"}
        </button>

        <div class="speed-group" role="group" aria-label="Simulation speed">
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
          Reset
        </button>
      </footer>

      {error() && (
        <div class="error" role="alert">
          <strong>Simulation failed to load</strong>
          <span>{error()}</span>
          <button type="button" onClick={() => window.location.reload()}>
            Retry
          </button>
        </div>
      )}
    </div>
  );
};

export default App;

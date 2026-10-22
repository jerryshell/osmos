import { createEffect, createSignal, onCleanup, onMount } from "solid-js";
import * as osmos from "../osmos-wasm";

const colorList = [
  "#FFF1DC",
  "#E8D5C4",
  "#EEEEEE",
  "#F7C8E0",
  "#DFFFD8",
  "#B4E4FF",
  "#95BDFF",
  "#F47C7C",
  "#7DB9B6",
  "#F0A04B",
];

const size = 900;
const maxSpeed = 100;
const simulator = new osmos.Simulator();

const clampSpeed = (value: number) =>
  Number.isFinite(value) ? Math.min(maxSpeed, Math.max(1, Math.floor(value))) : 1;

const App = () => {
  const [speedInput, setSpeedInput] = createSignal("1");
  const [speed, setSpeed] = createSignal(1);
  const [paused, setPaused] = createSignal(false);
  const [epoch, setEpoch] = createSignal(0);
  const [step, setStep] = createSignal(0);
  const [population, setPopulation] = createSignal(0);

  let canvas: HTMLCanvasElement | undefined;

  const applySpeed = () => {
    const next = clampSpeed(Number.parseInt(speedInput(), 10));
    setSpeed(next);
    setSpeedInput(String(next));
  };

  onMount(() => {
    if (!canvas) return;

    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    canvas.width = size * dpr;
    canvas.height = size * dpr;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    let frame = 0;

    const draw = () => {
      ctx.clearRect(0, 0, size, size);

      setEpoch(simulator.getEpochCount());
      setStep(simulator.getStepCount());

      const objectList = simulator.getObjectList();
      setPopulation(objectList.length);

      for (const object of objectList) {
        ctx.beginPath();
        ctx.fillStyle = colorList[object.id % colorList.length];
        ctx.arc(
          object.x * size,
          (1 - object.y) * size,
          object.energy,
          0,
          2 * Math.PI
        );
        ctx.fill();
      }
    };

    const render = () => {
      for (let i = 0; i < speed(); i++) {
        simulator.step();
      }
      draw();
      frame = requestAnimationFrame(render);
    };

    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      setPaused(true);
    }

    draw();

    createEffect(() => {
      if (paused()) {
        if (frame) cancelAnimationFrame(frame);
        frame = 0;
      } else if (!frame) {
        frame = requestAnimationFrame(render);
      }
    });

    onCleanup(() => {
      if (frame) cancelAnimationFrame(frame);
    });
  });

  return (
    <div id="app">
      <h1 class="visually-hidden">Osmos: neuroevolution simulation</h1>

      <header id="header">
        <div id="info">
          <span>🚀 Speed: {speed()}</span>{" "}
          <span>🧬 Epoch: {epoch()}</span>{" "}
          <span>🎮 Step: {step()}</span>{" "}
          <span>👾 Population: {population()}</span>
        </div>

        <form id="controls" onSubmit={(e) => { e.preventDefault(); applySpeed(); }}>
          <label class="visually-hidden" for="speed">
            Speed multiplier, 1 to {maxSpeed}
          </label>
          <input
            id="speed"
            name="speed"
            type="number"
            inputmode="numeric"
            autocomplete="off"
            min={1}
            max={maxSpeed}
            step={1}
            placeholder="Speed"
            value={speedInput()}
            onInput={(e) => setSpeedInput(e.currentTarget.value)}
          />
          <button type="submit">Set Speed</button>
          <button
            type="button"
            onClick={() => {
              setSpeed(1);
              setSpeedInput("1");
            }}
          >
            Reset Speed
          </button>
          <button
            type="button"
            aria-pressed={paused()}
            onClick={() => setPaused(!paused())}
          >
            {paused() ? "Resume" : "Pause"}
          </button>
        </form>
      </header>

      <main>
        <canvas
          ref={(el) => {
            canvas = el;
          }}
          role="img"
          aria-label="Live simulation of organisms evolving on a square field. Each circle is an organism; larger circles have more energy."
        >
          Your browser does not support the canvas element.
        </canvas>
      </main>

      <footer id="footer">
        <span>Author: </span>
        <a
          href="https://github.com/jerryshell"
          target="_blank"
          rel="noopener noreferrer"
        >
          @jerryshell
        </a>
      </footer>
    </div>
  );
};

export default App;

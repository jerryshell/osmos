import { createSignal, onCleanup, onMount } from "solid-js";
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
const simulator = new osmos.Simulator();

const App = () => {
  const [speedInput, setSpeedInput] = createSignal(1);
  const [speed, setSpeed] = createSignal(1);
  const [epoch, setEpoch] = createSignal(1);
  const [step, setStep] = createSignal(1);
  const [population, setPopulation] = createSignal(1);

  let canvas: HTMLCanvasElement;

  onMount(() => {
    const ctx = canvas!.getContext("2d");

    const render = () => {
      ctx!.clearRect(0, 0, size, size);

      setEpoch(simulator.getEpochCount());
      setStep(simulator.getStepCount());

      const objectList = simulator.getObjectList();
      setPopulation(objectList.length);

      for (let object of objectList) {
        ctx!.beginPath();
        ctx!.fillStyle = colorList[object.id % colorList.length];
        ctx!.arc(object.x * size, object.y * size, object.energy, 0, 2 * Math.PI);
        ctx!.fill();
      }

      for (let i = 0; i < speed(); i++) {
        simulator.step();
      }

      frame = requestAnimationFrame(render);
    };

    let frame = requestAnimationFrame(render);

    onCleanup(() => cancelAnimationFrame(frame));
  });

  return (
    <>
      <div id="app">
        <div id="header">
          <div id="info">
            <span>🚀 Speed: {speed()}</span>
            <span> · </span>
            <span>🧬 Epoch: {epoch()}</span>
            <span> · </span>
            <span>🎮 Step: {step()}</span>
            <span> · </span>
            <span>👾 Population: {population()}</span>
          </div>
          <input
            type="number"
            placeholder="Speed"
            value={speedInput()}
            onChange={(e) => setSpeedInput(e.target.valueAsNumber)}
          />
          <button onClick={() => setSpeed(speedInput())}>Set Speed</button>
          <button
            onClick={() => {
              setSpeed(1);
              setSpeedInput(1);
            }}
          >
            Reset Speed
          </button>
          <br />
        </div>

        <canvas ref={canvas!} width={size} height={size} />

        <div id="footer">
          <span>Author: </span>
          <a href="https://github.com/jerryshell" target="_blank">
            @jerryshell
          </a>
        </div>
      </div>
    </>
  );
};

export default App;

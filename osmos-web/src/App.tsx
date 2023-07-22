import { createSignal, onCleanup, onMount } from "solid-js";
import * as osmos from "../osmos-wasm";

const width = 1200;
const height = 600;
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

const sim = new osmos.Simulator(width, height);

const App = () => {
  const [speed, setSpeed] = createSignal(1);
  const [epoch, setEpoch] = createSignal(1);
  const [step, setStep] = createSignal(1);
  const [population, setPopulation] = createSignal(1);

  let speedInput: HTMLInputElement;
  let canvas: HTMLCanvasElement;

  const handleSetSpeedBtnClick = () => {
    setSpeed(speedInput.valueAsNumber);
  };

  const handleResetSpeedBtnClick = () => {
    setSpeed(1);
  };

  onMount(() => {
    const ctx = canvas.getContext("2d");

    const render = () => {
      ctx!.clearRect(0, 0, width, height);

      setEpoch(sim.getEpochCount());
      setStep(sim.getStepCount());

      const objectList = sim.getObjectList();
      setPopulation(objectList.length);

      for (let object of objectList) {
        ctx!.beginPath();
        ctx!.fillStyle = colorList[object.id % colorList.length];
        ctx!.arc(object.x, object.y, object.energy, 0, 2 * Math.PI);
        ctx!.fill();
      }

      for (let i = 0; i < speed(); i++) {
        sim.step();
      }

      frame = requestAnimationFrame(render);
    };

    let frame = requestAnimationFrame(render);

    onCleanup(() => cancelAnimationFrame(frame));
  });

  return (
    <div id="app">
      <div id="header">
        <div id="info">
          <span id="speedText">🚀 Speed: {speed()}</span>
          <span> · </span>
          <span id="epochText">🧬 Epoch: {epoch()}</span>
          <span> · </span>
          <span id="stepText">🎮 Step: {step()}</span>
          <span> · </span>
          <span id="populationText">👾 Population: {population()}</span>
        </div>
        <input
          id="speedInput"
          type="number"
          placeholder="Speed"
          ref={speedInput!}
        />
        <button id="setSpeedBtn" onClick={handleSetSpeedBtnClick}>
          Set Speed
        </button>
        <button id="resetSpeedBtn" onClick={handleResetSpeedBtnClick}>
          Reset Speed
        </button>
        <br />
      </div>

      <canvas ref={canvas!} width={width} height={height} />

      <div id="footer">
        <span>Author: </span>
        <a href="https://github.com/jerryshell" target="_blank">
          @jerryshell
        </a>
      </div>
    </div>
  );
};

export default App;

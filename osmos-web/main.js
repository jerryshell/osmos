import './style.css'
import * as osmos from './osmos-wasm'

let speed = 1
const speedText = document.getElementById('speedText')
const speedInput = document.getElementById('speedInput')
const setSpeedBtn = document.getElementById('setSpeedBtn')
setSpeedBtn.onclick = () => {
  speed = speedInput.value
  speedText.textContent = `Speed: ${speed}`
}
const resetSpeedBtn = document.getElementById('resetSpeedBtn')
resetSpeedBtn.onclick = () => {
  speed = 1
  speedText.textContent = `Speed: ${speed}`
}

const epochText = document.getElementById('epochText')

let sim = new osmos.Simulator()

const canvasElement = document.getElementById('canvas')
const width = 1000
const height = 1000
canvasElement.width = width
canvasElement.height = height

const ctx = canvasElement.getContext('2d')

const render = () => {
  ctx.clearRect(0, 0, width, height)
  const objectList = sim.getObjectList()
  for (let object of objectList) {
    ctx.beginPath();
    ctx.fillStyle = '#F47C7C'
    // console.table(object)
    ctx.arc(object.x * width, object.y * height, object.energy, 0, 2 * Math.PI);
    ctx.fill();
  }
  for (let i = 0; i < speed; i++) {
    sim.step()
  }
  const epochCount = sim.getEpochCount()
  const stepCount = sim.getStepCount()
  console.log('epochCount', epochCount, 'stepCount', stepCount, 'objectList.lenght', objectList.length)
  epochText.textContent = `Epoch: ${epochCount}`
  requestAnimationFrame(render)
}

render()

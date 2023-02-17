import './style.css'
import * as osmos from './osmos-wasm'

let speed = 1
const speedText = document.getElementById('speedText')
const speedInput = document.getElementById('speedInput')
const setSpeedBtn = document.getElementById('setSpeedBtn')
setSpeedBtn.onclick = () => {
  speed = speedInput.value
  speedText.textContent = `🚀 Speed: ${speed}`
}
const resetSpeedBtn = document.getElementById('resetSpeedBtn')
resetSpeedBtn.onclick = () => {
  speed = 1
  speedText.textContent = `🚀 Speed: ${speed}`
}

const epochText = document.getElementById('epochText')
const stepText = document.getElementById('stepText')
const populationText = document.getElementById('populationText')

const sim = new osmos.Simulator()

const width = 1000
const height = 1000

const canvasElement = document.getElementById('canvas')
canvasElement.width = width
canvasElement.height = height

const ctx = canvasElement.getContext('2d')

const render = () => {
  ctx.clearRect(0, 0, width, height)

  const objectList = sim.getObjectList()
  for (let object of objectList) {
    ctx.beginPath()
    ctx.fillStyle = '#F47C7C'
    ctx.arc(object.x * width, object.y * height, object.energy, 0, 2 * Math.PI)
    ctx.fill()
  }
  for (let i = 0; i < speed; i++) {
    sim.step()
  }

  epochText.textContent = `🧬 Epoch: ${sim.getEpochCount()}`
  stepText.textContent = `🎮 Step: ${sim.getStepCount()}`
  populationText.textContent = `👾 Population: ${objectList.length}`

  requestAnimationFrame(render)
}

render()

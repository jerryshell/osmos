import './style.css'
import * as osmos from './osmos-wasm'

const colorList = [
  '#FFF1DC',
  '#E8D5C4',
  '#EEEEEE',
  '#F7C8E0',
  '#DFFFD8',
  '#B4E4FF',
  '#95BDFF',
  '#F47C7C',
  '#7DB9B6',
  '#F0A04B',
]

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

const width = 1200
const height = 600

const canvasElement = document.getElementById('canvas')
canvasElement.width = width
canvasElement.height = height

const ctx = canvasElement.getContext('2d')

const sim = new osmos.Simulator(width, height)

const render = () => {
  ctx.clearRect(0, 0, width, height)

  const objectList = sim.getObjectList()
  for (let object of objectList) {
    ctx.beginPath()
    ctx.fillStyle = colorList[object.id % colorList.length]
    ctx.arc(object.x, object.y, object.energy, 0, 2 * Math.PI)
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

import './style.css'
import * as osmos from './osmos-wasm'

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
    ctx.fillStyle = 'rgb(0,255,128)'
    // console.table(object)
    ctx.arc(object.x * width, object.y * height, object.energy, 0, 2 * Math.PI);
    ctx.fill();
  }
  sim.step()
  const stepCount = sim.getStepCount()
  console.log('stepCount', stepCount, 'objectList.lenght', objectList.length)
  requestAnimationFrame(render)
}

render()

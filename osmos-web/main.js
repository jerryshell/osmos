import './style.css'
import * as osmos from './osmos-wasm'

const world = new osmos.World()

const canvasElement = document.getElementById('canvas')
const width = 1000
const height = 1000
canvasElement.width = width
canvasElement.height = height

const ctx = canvasElement.getContext('2d')

const render = () => {
  ctx.clearRect(0, 0, width, height)
  const cellList = world.cell_list()
  for (let cell of cellList) {
    ctx.beginPath();
    ctx.fillStyle = 'rgb(0,255,128)'
    // console.table(cell)
    ctx.arc(cell.x * width, cell.y * height, cell.energy, 0, 2 * Math.PI);
    ctx.fill();
  }
  world.step()
  requestAnimationFrame(render)
  // console.table(cellList[0].network_output)
}

render()

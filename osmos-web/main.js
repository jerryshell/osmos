import './style.css'
import * as osmos from './osmos-wasm'

const world = new osmos.World()

const canvasElement = document.getElementById('canvas')
canvasElement.width = window.innerWidth
canvasElement.height = window.innerHeight

const ctx = canvasElement.getContext('2d')

const render = () => {
  ctx.clearRect(0, 0, window.innerWidth, window.innerHeight)
  const cellList = world.cell_list()
  for (let cell of cellList) {
    ctx.beginPath();
    ctx.fillStyle = 'rgb(0,255,128)'
    // console.table(cell)
    ctx.arc(cell.x * window.innerWidth, cell.y * window.innerHeight, cell.energy, 0, 2 * Math.PI);
    ctx.fill();
  }
  world.step()
  requestAnimationFrame(render)
  // console.table(cellList[0].network_output)
}

render()

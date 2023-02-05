import { hello } from './osmos-wasm'

export function setupCounter(element) {
  let counter = 0
  const setCounter = (count) => {
    counter = count
    element.innerHTML = `count is ${counter}`
  }
  element.addEventListener('click', () => {
    alert(hello())
    setCounter(counter + 1)
  })
  setCounter(0)
}

# Osmos

Rust + WASM + 神经网络 + 遗传算法

Bilibili 视频：*TODO*

## 如何运行

第 0 步，安装 `wasm-pack`

[https://rustwasm.github.io/wasm-pack](https://rustwasm.github.io/wasm-pack)

第 1 步，编译 `osmos-wasm`

```bash
cd osmos-wasm
wasm-pack build --out-dir ../osmos-web/osmos-wasm
```

第 2 步，进入 `osmos-web` 安装依赖并运行

```bash
cd osmos-web
yarn
yarn dev
```

## Vite WASM Note

1. Install `vite-plugin-wasm`

```bash
yarn add -D vite-plugin-wasm
```

2. Edit `vite.config.js` file

```js
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
    plugins: [
        wasm(),
    ]
})
```

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)

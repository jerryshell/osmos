# Osmos

Rust + WASM + 神经网络 + 遗传算法

Bilibili 视频：*TODO*

## Note

```bash
wasm-pack build --out-dir ../osmos-web/osmos-wasm
```

```bash
yarn add -D vite-plugin-wasm
```

```js
// vite.config.js
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
    plugins: [
        wasm(),
    ]
})
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)

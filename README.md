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

## 目录简介

```
osmos
├── osmos-core # 核心数据结构
├── osmos-ga   # 遗传算法
├── osmos-nn   # 神经网络
├── osmos-sim  # 进化模拟器，使用类似 ECS 的模式驱动全局数据
│   └── src
│       └── system
│           ├── collision.rs # 碰撞系统
│           ├── movement.rs  # 移动系统
│           ├── network.rs   # 神经网络系统
│           └── sensor.rs    # 感知器系统
├── osmos-wasm # 将模拟器编译为 WASM，代理模式
└── osmos-web  # Web UI，通过导入 WASM 启动模拟器，并将模拟器的数据渲染到 Canvas 中
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

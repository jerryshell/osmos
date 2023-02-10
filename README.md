# Osmos

Demo: [osmos.jerryshell.eu.org](https://osmos.jerryshell.eu.org)

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

## 项目结构简介

```
osmos
├── osmos-core # 核心数据结构
├── osmos-nn   # 神经网络
├── osmos-sim  # 进化模拟器，使用类似 ECS 的模式驱动全局数据
│   └── src
│       ├── ga # 遗传算法
│       │   ├── crossover.rs # 杂交
│       │   ├── evolve.rs    # 进化
│       │   ├── fitness.rs   # 适应度
│       │   ├── gene.rs      # 基因
│       │   ├── mutation.rs  # 变异
│       │   └── selection.rs # 选择
│       └── system # ECS 子系统
│           ├── collision.rs # 碰撞系统
│           ├── movement.rs  # 移动系统
│           ├── network.rs   # 神经网络系统
│           └── sensor.rs    # 感知器系统
├── osmos-wasm # 将模拟器编译为 WASM，代理模式
└── osmos-web  # Web UI，通过导入 WASM 启动模拟器，并将模拟器的数据渲染到 Canvas 中
```

## Vite WASM

1. Install `vite-plugin-wasm` & `vite-plugin-top-level-await`

```bash
yarn add -D vite-plugin-wasm vite-plugin-top-level-await 
```

2. Edit `vite.config.js`

```js
import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait(),
    ]
})
```

## ⚠️ Rust WASM Warning

Don't use [wee_alloc](https://github.com/rustwasm/wee_alloc) in production code.

* [r/rust: dont_use_wee_alloc_in_production_code_targeting](https://www.reddit.com/r/rust/comments/x1cle0/dont_use_wee_alloc_in_production_code_targeting)
* [wee_alloc/issues: #106 Unbounded Memory Leak](https://github.com/rustwasm/wee_alloc/issues/106)
* [wee_alloc/issues: #107 Is this repo still maintained?](https://github.com/rustwasm/wee_alloc/issues/107)
* [RUSTSEC-2022-0054: wee_alloc is Unmaintained](https://rustsec.org/advisories/RUSTSEC-2022-0054.html)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)

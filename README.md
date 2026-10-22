# Osmos

Rust + WASM + Vite + 质量生态

当前进度是 Phase 6 预览。世界包含质量、空间网格、16 扇区感知、55-8-2 控制网络、质量成本推进、喷出物、猎手、猎物、场景和遗传算法。Web 支持质体检查面板和场景切换。学习检查使用隔离回合。

预览地址：<https://osmos.jerryshell.eu.org/next/>。生产根站仍是旧版，切换待确认。

旧版演示：[BV1vj411A7k2](https://www.bilibili.com/video/BV1vj411A7k2)

立刻体验：[osmos.jerryshell.eu.org](https://osmos.jerryshell.eu.org)

## 如何运行

1. 安装 `wasm-pack` 和 wasm32 目标

```bash
rustup target add wasm32-unknown-unknown
```

[https://wasm-bindgen.github.io/wasm-pack/](https://wasm-bindgen.github.io/wasm-pack/)

2. 编译 `osmos-wasm`

```bash
cd osmos-wasm
wasm-pack build --out-dir ../osmos-web/osmos-wasm
```

3. 进入 `osmos-web` 安装依赖并运行

```bash
cd osmos-web
npm install
npm run dev
```

## 生产构建

```bash
cd osmos-web
npm run check       # TypeScript 类型检查 + Oxlint
npm run build:all   # wasm-pack release + wasm-opt，然后 Vite 构建到 dist/
```

也可以分开执行：`npm run build:wasm` 和 `npm run build`。

## 项目结构

```
osmos
├── osmos-core # ID、向量、质量、半径
├── osmos-sim  # 世界、质体、颗粒、碰撞、空间网格、快照
├── osmos-evo  # 控制网络、遗传算法、Trainer、共享世界 Engine
├── osmos-wasm # 扁平快照 ABI
├── osmos-headless # 训练、基准、留出种子复评
└── osmos-web  # SolidJS + Canvas 2D
```

`osmos-sim` 默认生成 64 个质体和 236 个颗粒，一共 300 个实体。仿真以 60 Hz 固定步长推进，内部使用 2 个物理子步和 4 × 4 空间网格。控制网络拓扑为 55-8-2，基因组长度 466。

训练命令。

```bash
cargo run --release -p osmos-headless -- --train --generations 60 --seed 42 --holdout 10
```

## 开源协议

[GNU Affero General Public License v3.0](LICENSE)

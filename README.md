# Osmos

Rust + WASM + Vite + 质量生态

当前进度是 Phase 1。世界包含质量、吞噬、空间网格、固定步长和扁平快照。感知、控制网络、喷射推进和遗传算法在后续阶段加入。

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
├── osmos-evo  # 引擎外壳；遗传算法在后续阶段加入
├── osmos-wasm # 扁平快照 ABI
└── osmos-web  # SolidJS + Canvas 2D
```

`osmos-sim` 默认生成 64 个质体和 236 个颗粒，一共 300 个实体。仿真以 60 Hz 固定步长推进，内部使用 2 个物理子步和 4 × 4 空间网格。

## 开源协议

[GNU Affero General Public License v3.0](LICENSE)

# Osmos

Rust + WASM + Vite + 神经网络 + 遗传算法

立刻体验：[https://osmos.jerryshell.eu.org](https://osmos.jerryshell.eu.org)

Bilibili 视频：[BV1vj411A7k2](https://www.bilibili.com/video/BV1vj411A7k2)

## 如何运行

1. 安装 `wasm-pack`

[https://rustwasm.github.io/wasm-pack](https://rustwasm.github.io/wasm-pack)

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

## 项目结构简介

```
osmos
├── osmos-core # 核心数据结构
├── osmos-nn   # 神经网络
├── osmos-sim  # 进化模拟器，使用类似 ECS 的模式驱动全局数据
│   └── src
│       ├── ga # 遗传算法
│       │   ├── crossover.rs # 交叉
│       │   ├── evolve.rs    # 进化
│       │   ├── fitness.rs   # 适应度
│       │   ├── gene.rs      # 基因
│       │   ├── mutation.rs  # 变异
│       │   └── selection.rs # 选择
│       └── system # ECS 子系统
│           ├── collision.rs # 碰撞系统
│           ├── epoch.rs     # 迭代系统
│           ├── movement.rs  # 移动系统
│           ├── network.rs   # 神经网络系统
│           └── sensor.rs    # 感知器系统
├── osmos-wasm # 将模拟器编译为 WASM，代理模式
└── osmos-web  # Web UI，通过导入 WASM 启动模拟器，并将模拟器的数据渲染到 Canvas 中
```

## 开源协议

[GNU Affero General Public License v3.0](LICENSE)

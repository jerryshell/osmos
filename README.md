# Osmos

Osmos 是一个零玩家质量模拟项目，使用 Rust、WebAssembly、Vite 和 Canvas 2D 构建。项目关注质量、动量、吞噬、环绕边界和遗传算法共同形成的多质体行为。

当前版本为预览版。生产站点仍为旧版，切换尚未确认。

- 预览：<https://osmos.jerryshell.eu.org/next/>
- 生产：<https://osmos.jerryshell.eu.org/>

## 当前实现

- 64 个质体全部是学习者，不生成脚本猎手或脚本猎物。
- 浏览器启动时加载 `osmos-web/src/champion.json` 中的冠军基因组。8 个精确副本保留训练策略，其余副本变异后继续演化。
- 质量较大者渐进吸收较小者，没有 2 倍门槛。质量相同时弹开。
- 吸收比例按重叠面积计算，每 60 Hz tick 转移一次。
- 地图边界环绕。移动、碰撞、吸收、场力和感知都使用最短环绕距离。
- 颗粒储备为 `120` 质量。颗粒少于 `236` 个时从储备补充，储备耗尽后停止。
- 控制网络为 `55-8-2`，基因组长度为 466。
- 场景包括环境、感知、轨道和斥力。
- 轨道和斥力场景使用中心场力。正强度吸引，负强度排斥。场力指数默认是 2。
- 浏览器提供质体检查面板、场景切换、暂停、调速和重置。
- 界面支持英语和简体中文。语言可由 URL、浏览器语言或本地设置决定。

## 运行方式

安装 `wasm32-unknown-unknown` 目标。

```bash
rustup target add wasm32-unknown-unknown
```

安装 `wasm-pack`。

<https://wasm-bindgen.github.io/wasm-pack/>

编译 `osmos-wasm`。

```bash
cd osmos-wasm
wasm-pack build --out-dir ../osmos-web/osmos-wasm
```

安装依赖并启动 Web。

```bash
cd osmos-web
npm install
npm run dev
```

## 生产构建

```bash
cd osmos-web
npm run check
npm run build:all
```

`npm run check` 执行 TypeScript 类型检查和 Oxlint。`npm run build:all` 执行 `wasm-pack` 发布构建和 `wasm-opt`，然后由 Vite 生成 `dist/`。

也可以分开执行 `npm run build:wasm` 和 `npm run build`。

## 项目结构

- `osmos-core`：ID、向量、质量、半径
- `osmos-sim`：世界、质体、颗粒、碰撞、空间网格、快照
- `osmos-evo`：控制网络、遗传算法、多质体训练、共享世界
- `osmos-wasm`：扁平快照 ABI
- `osmos-headless`：训练、基准、留出种子复评
- `osmos-web`：SolidJS 与 Canvas 2D

默认世界包含 64 个质体和 236 个颗粒，共 300 个实体。`Orbital` 和 `Repulsor` 场景包含 64 个质体、180 个颗粒和一口中心场力井。

仿真以 60 Hz 固定步长推进，内部使用 2 个物理子步和 8 × 8 空间网格。

## 训练

全学习者训练。

```bash
cargo run --release -p osmos-headless -- --ecology --generations 60 --seed 42 --motes 64 --pellets 236 --episode 2400
```

全学习者路径每局让 64 个质体同场竞争。对局结束后按最终质量计算繁殖权重，死亡质体权重为 0。亲本通过质量概率选择，子代经过交叉和变异。

导出浏览器冠军基因组。

```bash
cargo run --release -p osmos-headless -- --generations 60 --seed 42 --motes 64 --pellets 236 --episode 2400 --export-champion osmos-web/src/champion.json
```

## 文档

- `docs/architecture.md`：世界、质量、场力、感知、训练和浏览器结构
- `docs/training.md`：全学习者训练、选择方式和当前结果
- `docs/verification.md`：构建、浏览器、性能和语言验证
- `docs/release.md`：发布门禁、浸泡、跨浏览器和生产切换
- `CHANGELOG.md`：版本更新记录
- `docs/i18n.md`：英语和简体中文国际化

## 开源协议

[MIT](LICENSE)

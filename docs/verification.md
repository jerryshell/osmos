# 验证

## 构建检查

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd osmos-web
npm run check
npm run build:all
```

当前结果。

- Rust 测试 50 个通过。core 6 个，evo 10 个，sim 34 个。
- 格式检查、Clippy、TypeScript 和 Oxlint 通过。
- 1,000 步 Ambient 哈希为 `65b791be50513d5b`。

## 浏览器

- 60 秒测试 8 个组合，覆盖桌面、手机、1×、4×、8×、Orbital 和 Repulsor。
- 冠军基因组加载标记为 `true`。
- 帧率 59.58 到 60.02 fps。
- 8× 桌面 `step` p95 为 6.3 ms，手机为 6.2 ms。
- Orbital 手机 4× 为 60.00 fps，`step` p95 为 2.5 ms。
- Repulsor 手机 4× 为 60.02 fps，`step` p95 为 2.3 ms。
- 页面错误为 0。
- 所有按钮不小于 44 px。

10 分钟桌面 8× 浸泡。

- 119 代，35,500 帧，59.17 fps。
- `step` p95 为 6.4 ms，`frame` p95 为 16.8 ms。
- 24 次长任务，页面错误为 0。
- JS 堆从 3.1 MB 增长到 9.9 MB。未做堆快照，不能据此排除泄漏。

## 语言

交互测试确认以下行为。

- 中文界面使用 `html lang=zh-Hans`，HUD 显示 `最佳质量`，URL 写入 `lang=zh`。
- 英文界面使用 `html lang=en`，HUD 显示 `BEST MASS`，URL 写入 `lang=en`。
- 语言切换后页面错误为 0。

## 性能与体积

- 500 个实体运行 2,400 步耗时 1.21 秒，约 1,983 步/秒。
- wasm 102.09 KB，gzip 42.96 KB。
- JavaScript 38.33 KB，gzip 16.02 KB。
- CSS 6.22 KB，gzip 1.79 KB。

## 跨浏览器

- Chromium 手机 1×，5 秒，60 fps，页面错误 0。
- Firefox 手机 1×，5 秒，59.8 fps，页面错误 0。
- WebKit 手机 1×，5 秒，36.8 fps，页面错误 0。Linux 无头 WebKit 性能低于 Chromium，未做真机验证。

## 持续集成

GitHub Actions 运行成功。

- 工作流：`.github/workflows/ci.yml`
- 运行：<https://github.com/jerryshell/osmos/actions/runs/39931590182>

## 预览

<https://osmos.jerryshell.eu.org/>

生产站点已切换为 `v2.0.0`。预览站点 `/next/` 使用同一构建。

## 未验证

- Safari 真机、真实移动网络和触摸操作。
- 生产切换和回滚演练。
- WebKit 的 60 fps 目标。

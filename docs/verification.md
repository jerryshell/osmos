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

- Rust 测试 50 个通过。core 6 个，evo 12 个，sim 32 个。
- 格式检查、Clippy、TypeScript 和 Oxlint 通过。
- 1,000 步 Ambient 哈希为 `3f908b96443bc945`。

## 浏览器

- 60 秒测试 8 个组合，覆盖桌面、手机、1×、4×、8×、Orbital 和 Repulsor。
- 冠军基因组加载标记为 `true`。
- 帧率 59.62 到 60.02 fps。
- 8× 桌面 `step` p95 为 6.6 ms，手机为 5.6 ms。
- Orbital 手机 4× 为 60.02 fps，`step` p95 为 2.2 ms。
- Repulsor 手机 4× 为 60.00 fps，`step` p95 为 2.1 ms。
- 页面错误为 0。
- 所有按钮不小于 44 px。

10 分钟桌面 8× 浸泡。

- 119 代，35,078 帧，58.46 fps。
- `step` p95 为 7.8 ms，`frame` p95 为 16.8 ms。
- 36 次长任务，页面错误为 0。
- JS 堆从 3.1 MB 增长到 9.3 MB。未做堆快照，不能据此排除泄漏。

## 语言

交互测试确认以下行为。

- 中文界面使用 `html lang=zh-Hans`，HUD 显示 `最佳质量`，URL 写入 `lang=zh`。
- 英文界面使用 `html lang=en`，HUD 显示 `BEST MASS`，URL 写入 `lang=en`。
- 语言切换后页面错误为 0。

## 性能与体积

- 500 个实体运行 2,400 步耗时 0.79 秒，约 3,038 步/秒。
- wasm 110.06 KB，gzip 45.93 KB。
- JavaScript 38.70 KB，gzip 16.14 KB。
- CSS 6.22 KB，gzip 1.79 KB。

## 跨浏览器

- Chromium 手机 1×，5 秒，60 fps，页面错误 0。
- Firefox 手机 1×，5 秒，60 fps，页面错误 0。
- WebKit 手机 1×，5 秒，44.6 fps，页面错误 0。Linux 无头 WebKit 性能低于 Chromium，未做真机验证。

## 预览

<https://osmos.jerryshell.eu.org/next/>

生产站点仍为旧版，切换尚未执行。

## 未验证

- Safari 真机、真实移动网络和触摸操作。
- GitHub Actions 的远端运行结果。
- 生产切换和回滚演练。
- WebKit 的 60 fps 目标。

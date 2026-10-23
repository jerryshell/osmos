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

- Rust 测试 45 个通过。core 6 个，evo 10 个，sim 29 个。
- 格式检查、Clippy、TypeScript 和 Oxlint 通过。
- 1,000 步 Ambient 哈希为 `0b058a3f390d9701`。

## 浏览器

- 60 秒测试 8 个组合，覆盖桌面、手机、1×、4×、8×、Orbital 和 Repulsor。
- 帧率 58.23 到 59.97 fps。
- 8× 桌面 `step` p95 为 7.8 ms，手机为 8.4 ms。
- Orbital 手机 4× 为 59.93 fps，`step` p95 为 4.4 ms。
- Repulsor 手机 4× 为 59.97 fps，`step` p95 为 4.3 ms。
- 页面错误为 0。
- 所有按钮不小于 44 px。

## 语言

交互测试确认以下行为。

- 中文界面使用 `html lang=zh-Hans`，HUD 显示 `最佳质量`，URL 写入 `lang=zh`。
- 英文界面使用 `html lang=en`，HUD 显示 `BEST MASS`，URL 写入 `lang=en`。
- 语言切换后页面错误为 0。

## 性能与体积

- 500 个实体运行 2,400 步耗时 0.98 秒，约 2,449 步/秒。
- wasm 106.11 KB，gzip 44.08 KB。
- JavaScript 33.28 KB，gzip 13.03 KB。
- CSS 6.22 KB，gzip 1.79 KB。

## 预览

<https://osmos.jerryshell.eu.org/next/>

生产站点仍为旧版，切换尚未执行。

## 未验证

- Firefox、Safari 和真机。
- GitHub Actions，分支尚未推送。
- 生产切换和回滚演练。

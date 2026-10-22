# Phase 5：加固

- 日期：2026-10-22
- 状态：部分完成
- 分支：`rewrite/osmos-v2`
- 范围：测试、性能、内存、wasm 体积、CI、预览

## 1. 已完成

- Rust 测试 32 个通过。`osmos-core` 5 个，`osmos-evo` 8 个，`osmos-sim` 19 个。
- `cargo fmt --check`、Clippy、TypeScript、Oxlint 和 Vite 构建通过。
- 60 秒浏览器场景 6 个，1×、4×、8×，桌面和手机视口。帧率 59.83 到 60.00 fps，长任务为 0。
- 8× 下每帧 `step` 加 `snapshot` 的 p95 约 5.5 ms，低于 8 ms 的 Worker 触发线。
- 500 个实体，64 个 AI 加 436 个颗粒，2,400 步耗时 0.76 到 0.85 秒，约 2,824 到 3,158 步/秒。
- wasm 108.36 KB，gzip 45.74 KB。JavaScript 27.25 KB，gzip 10.69 KB。CSS 6.11 KB，gzip 1.76 KB。
- `/next/` 预览部署完成，旧根站保持 200。

## 2. 10 分钟连续运行

Playwright 在手机视口运行 8×，每 30 秒采样一次 `performance.memory.usedJSHeapSize` 和 HUD。

结果见 `docs/phase-6/soak.jsonl`。10 分钟完成 119 代。堆内存从 3.04 MB 开始，在 121 秒达到 11.30 MB，结束时 3.87 MB。出现多次回收，没有单调增长。页面错误为 0。

## 3. 未完成

- Firefox 和 Safari 没有运行。当前只有系统 Chrome。
- GitHub Actions 没有运行，因为分支没有推送。
- 真机和云真机没有测试。
- native 与 wasm 的 1,000 步对齐仍未完成。
- wasm 10 代趋势没有单独记录。

## 4. 结论

当前构建满足本地性能预算，可以作为预览。生产切换仍需要 Phase 6 的单独确认。

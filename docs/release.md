# 发布检查

## 当前状态

- 生产版：<https://osmos.jerryshell.eu.org/>，`v2.0.1` 已上线。
- 预览版：<https://osmos.jerryshell.eu.org/next/>，同一构建。
- 当前定位：`v2.0.1` 正式版。
- CI：已通过。
- 浸泡：600 秒桌面 8× 通过，`v2.0.0` 记录。
- 跨浏览器：Firefox 通过，WebKit 功能通过但帧率低于 60，`v2.0.0` 记录。
- 站点托管：1Panel 站点类型 `static`，内容在 `/www/sites/osmos.jerryshell.eu.org/index/`。
- 线上构建：提交 `c500b25`，wasm 102.09 KB，JavaScript 38.33 KB。

## 发布门禁

按顺序执行。

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd osmos-web
npm run check
npm run build:all
```

浏览器验证。

```bash
CHROME_PATH=/usr/bin/google-chrome VERIFY_SECONDS=60 npm run verify:ui
CHROME_PATH=/usr/bin/google-chrome VERIFY_INTERACTIONS=1 npm run verify:ui
```

单场景浸泡。

```bash
CHROME_PATH=/usr/bin/google-chrome VERIFY_SECONDS=600 VERIFY_CASE=desktop-8x npm run verify:ui
```

跨浏览器冒烟。

```bash
npx playwright install firefox webkit
VERIFY_SECONDS=5 VERIFY_BROWSER=firefox VERIFY_CASE=mobile-1x npm run verify:ui
VERIFY_SECONDS=5 VERIFY_BROWSER=webkit VERIFY_CASE=mobile-1x npm run verify:ui
```

## 生产切换

切换前必须备份当前站点和 vhost，并确认回滚命令。切换生产根站属于不可逆操作，需要单独确认。

1. 记录当前生产文件和 vhost。
2. 生成预览构建。
3. 将生产根站指向新构建。
4. 检查首页、场景切换、语言切换和错误日志。
5. 如果检查失败，恢复备份并重新加载服务。

## 未验证

- Safari 真机和真实移动网络。
- 生产切换和回滚演练。
- WebKit 的 60 fps 目标。

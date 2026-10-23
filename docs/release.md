# 发布检查

## 当前状态

- 预览版：<https://osmos.jerryshell.eu.org/next/>
- 生产站点：仍为旧版，切换需要单独确认。
- 当前定位：预览候选，不是正式版。

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
- GitHub Actions 的远端运行结果。
- 生产切换和回滚演练。

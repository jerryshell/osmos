# Phase 6：预览与切换

- 日期：2026-10-22
- 状态：预览已部署，生产切换待确认
- 分支：`rewrite/osmos-v2`
- 预览地址：<https://osmos.jerryshell.eu.org/next/>

## 1. 预览

预览构建使用 Vite `--base=/next/`，产物在 `osmos-web/dist-preview/`，部署到：

```text
/opt/1panel/www/sites/osmos.jerryshell.eu.org/preview/next/
```

Nginx 增加三个位置。

- `/next` 重定向到 `/next/`。
- `/next/assets/` 使用 1 年 immutable 缓存。
- `/next/` 使用 no-cache，SPA fallback 到 `/next/index.html`。

验证结果。

- `https://osmos.jerryshell.eu.org/next/` 返回 200。
- JavaScript 返回 `application/javascript` 和 immutable 缓存。
- wasm 返回 `application/wasm`、gzip 和 immutable 缓存。
- 旧根站 `https://osmos.jerryshell.eu.org/` 仍返回 200。
- 浏览器加载预览后没有页面错误。

## 2. 回滚

预览配置修改前的 vhost 备份在 `docs/phase-6/vhost-before-preview.conf`。当前带预览位置的配置快照在 `docs/phase-6/vhost-with-preview.conf`。

回滚步骤。

1. 恢复 vhost 备份。
2. 执行 `nginx -t` 和 `nginx -s reload`。
3. 删除 `/opt/1panel/www/sites/osmos.jerryshell.eu.org/preview/`。

根站未被修改，回滚不影响旧版本。

## 3. 生产切换

生产切换会替换根目录：

```text
/opt/1panel/www/sites/osmos.jerryshell.eu.org/index
```

步骤。

1. 用根路径重建 `dist`，确认 `base` 为 `/`。
2. 备份旧根目录到 `index.before-v2-20261022`。
3. 使用 `rsync -a --delete` 部署新 `dist`。
4. 验证 HTTPS、gzip、缓存头、wasm MIME 和 SPA fallback。
5. 在桌面和手机打开根站。
6. 保留旧目录 7 天。

切换尚未执行。生产部署需要单独确认。

## 4. 验收

预览完成后检查以下内容。

- 根站旧版本正常。
- 预览页 200，控制台无错误。
- 场景切换、检查面板、暂停、调速和 Reset 可用。
- wasm 和 assets 缓存头正确。
- 回滚备份存在。

## 5. 剩余风险

- 真实手机和跨浏览器没有验证。
- GitHub Actions 没有运行。
- 学习门禁为 2/3，种子 44 失败。
- native 与 wasm 对齐未完成。
- 预览路径与根路径的静态资源 base 不同，切换时必须使用根路径构建。

# TermForge Design System — 使用指南（Guidelines）

> 版本：v1.0（2026-09-02，P5 产物）
> 面向：V1 新版原型与重开发实现（Tauri 2 + Svelte 4 延续方案，见 P7 docs/04_architecture/SYSTEM_ARCH.md）。

---

## 1. 总原则

1. **令牌是唯一事实源**：颜色/间距/字号/布局尺寸一律引用 CSS 变量（TOKEN.md），组件内不得出现硬编码色值。终端主题色同样运行时读令牌（`readThemeFromTokens` 模式，TerminalTab L51-58）。
2. **组件优先业务化**：新增界面先用 COMPONENT.md 记录表内的组件拼装；缺组件先入 DS 再上界面。
3. **不虚构能力**：规划功能（PRD F029-F046 中未实现项）一律走引导式空态 + C 类决策清单标注，UI 不得出现无后端支撑的可用假象。
4. **源码缺陷按修复后规格呈现**：涉及三项已注明的修复后规格——Ctrl+R 死文案（B-04）、closed 态重连入口（B-05）、加密失败拒绝落盘（C-7 呈现层）；其余行为保持旧项目基线。

## 2. 命名与代码组织约定

- CSS 变量：`--{类别}-{名称}`（bg-/fg-/space-/text-/radius- 前缀），新增令牌先入 TOKEN.md。
- 组件文件：业务组件 `components/`，跨视图基础件 `components/primitives/`，布局件 `components/layout/`（沿用旧结构，见 P7 docs/04_architecture/MODULE_ARCH.md）。
- 图标：统一入 `lib/icons.ts` 内联 SVG（ASSETS.md 规范），禁止外链图标 CDN。
- 状态文案：五态标签文案全局唯一（Idle/Connecting.../Connected/Error/Disconnected），不得在别处另造同义文案。

## 3. 可用性基线（accessibility，来自源码事实 + V1 延续）

- Toast 容器 `aria-live="polite"` + `role="status"`（保留）。
- Tab：`role="tab"` + `tabindex="0"` + `aria-selected` + Enter/Space 激活（保留）。
- 已存连接项：`role="button"` + `tabindex="0"` + Enter（保留）。
- 连接表单：label 与 input 以 id 关联（`for`/`id`，保留）；`role="form"`。
- 危险确认与指纹确认（V1）：模态打开时焦点入面板、Esc=取消、焦点默认安全侧（取消按钮）。
- 对比度说明：Tokyo Night 色板为既定视觉基线（fg-secondary #565f89 在深底上的对比度有限，用于次级/辅助信息——事实记录，弱对比仅承载非关键信息）。

## 4. 文案规范（从源码提炼）

- 语言：界面文案英文（旧项目事实），文档与评审面板中文；错误提示句式「原因 — 处置建议」（如 "Connection refused — check host and port"）。
- 终端系统行前缀：`[status]` / `[error]`（保留）。
- 按钮：动词或「动词+宾语」（Connect/Save/Reconnect/Delete）；危险确认的确认按钮用具体危险动词（删除），不用 OK/Yes。
- 占位能力一律显式标注「规划中/占位」，禁止暗示可用。

## 5. 禁止事项

- 禁止引入网络字体/CDN/外链图片（Tauri CSP `default-src 'self'` 亦不允许，tauri.conf.json）。
- 禁止在组件内 new 一套状态色映射（StatusDot 是唯一映射源——旧项目 TabStrip/StatusBar 两处重复定义即前车之鉴）。
- 禁止明文密码出现在日志/Debug 输出（沿用 dto.rs/store.rs 手工脱敏 Debug 模式）。
- 禁止用原生 `confirm()/alert()`（V1 起统一 DangerConfirm，B-06）。
- 禁止在快捷键提示中宣传未注册的组合键（Ctrl+R 教训，B-04）。

## 6. 评审与变更流程

1. 令牌/组件/模式变更：先改 DS 对应文件（tokens/components/patterns/assets），注明来源与版本，再落实现。
2. 每个新组件必须登记 COMPONENT.md §9 记录表（来源/状态/关联功能 ID）。
3. 原型层：V0（`prototype/v0-old/`）为旧项目事实基线快照，**只读不改**；V1（`prototype/v1-new/`）承载 B 类优化与 DS 应用；两版差异即「重开发体验增量」的直观对照。
4. 与 PRD 的对齐：DS 记录表「关联功能」列必须能映射到 PRD 功能 ID（F001-F046），保证可溯源。

## 7. 公共参数速查（详见 docs/07_design_system/TOKEN.md §8）

- 连接字段：id/name/host/port(1-65535,默认22)/username/password?/key_path?(UI 缺，F045 规划)。
- 五态枚举：`idle | connecting | connected | closed | error`。
- 快捷键 9 组：Ctrl+1..9 / Ctrl+T / Ctrl+W / Ctrl+Tab / Ctrl+Shift+Tab / Ctrl+\ / Ctrl+Shift+P / Ctrl+Shift+N / Esc。
- TOFU：`~/.termforge/known_hosts`，`host:port hex指纹`，首录信任（V1 确认式）、变更即拒 + MITM 警告。
- 加密：AES-256-GCM，密钥=SHA-256(`TermForge-v1:{hostname}:{username}`)，base64(nonce12+ct+tag16)，0600 落盘。
- 数据目录：`~/.termforge/`（connections.json / known_hosts）。
- 超时：前端连接 15s；TCP read 30s / write 10s（client.rs L125-126）。

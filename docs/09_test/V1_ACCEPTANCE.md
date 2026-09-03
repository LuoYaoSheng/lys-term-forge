# TermForge V1 新版原型验收报告

> 验收人角色：测试负责人（不修改产品逻辑；仅当发现原型实现缺陷时修复并记录修复项）
> 验收日期：2026-09-02
> 验收对象：`prototype/v1-new/app-prototype.html`（单文件可交互原型，1214 行）
> 对比基准：`docs/02_product/PRD.md`（46 功能）、`docs/06_review/PRODUCT_REVIEW.md`（13 项 B 类）、`docs/07_design_system/`（五文件）、`docs/02_product/PAGE_SPEC.md`（交互规格）
> 对照基线：`prototype/v0-old/app-prototype.html`（旧项目事实基线，只读）

---

## 一、验收方法

1. **静态校验**：提取原型内嵌 JS 执行 `node --check` —— **语法通过**（修复缺陷后复跑仍通过，见 §四）。
2. **浏览器实测**：`python3 -m http.server 8765` + Playwright 加载 `http://127.0.0.1:8765/v1-new/app-prototype.html`，执行 T1-T9 组用例（连接主链路、TOFU、断线重连、引导式空态）。
3. **无头 DOM 实测（jsdom）**：测试环境浏览器被并行自动化会话持续侵占（页签漂移至其他项目原型），为完成剩余用例改用 jsdom（Node 24 + jsdom 26）执行 T10-T14 与 V1 补充组共 35 项断言；所有脚本均带页面 URL 防御检查，未发生误测。
4. **DS 一致性静态核对**：`:root` 令牌与 `docs/07_design_system/TOKEN.md` 逐值比对（26 项）；零外链检查。
5. **控制台检查**：全程唯一报错为 `favicon.ico 404`（同 V0，非功能缺陷，file:// 直开不受影响）。
6. 测试完成后 http.server 已关闭。

## 二、功能覆盖检查（PRD 46 项 vs V1）

### 2.1 已实现功能 F001-F028（28 项，行为基线保留 + B 类增强）

| 功能 | V1 呈现 | 实测 |
|---|---|---|
| F001-F003 视图切换/侧栏折叠/拖宽 | 外壳保留；拖宽 180-400 钳制 | ✅ 浏览器 T1/T9（视图切换+引导跳转）|
| F004 表单校验 | 三规则 + 失焦 touched | ✅ jsdom T12（空表单 2 错误）|
| F005 发起连接 | Connect/Enter/卡片快连/双击 | ✅ T2/T12b/T14 |
| F006 保存（查重+加密说明） | 查重拦截 + Toast 注明 AES-256-GCM 0600 | ✅ jsdom T12 |
| F007 删除（危险确认） | **B-06 自绘 DangerConfirm**，取消默认焦点 | ✅ jsdom T11（取消/确认两分支）|
| F008/F009 回填/双击直连 | 单击回填 + 双击直连 | ✅ jsdom T14 |
| F010-F013 Tab 新建/关闭/切换/重命名 | 保留；**B-11 「新建连接」语义** | ✅ jsdom T13（重命名 Enter 生效）|
| F014-F016 连接流程/输入/输出 | 15s 超时语义场景 + 命令回显 | ✅ T2/T3/T5 |
| F017/F018 PTY 同步/字号 | 状态栏 cols×rows 随字号变化 + 9 档菜单 | ✅（呈现与 V0 同构）|
| F019 错误映射 | **7 条（B-13 新增密钥变更专案）** | ✅ V1 补充组（超时/认证/无密钥）|
| F020 手动重连 | **closed 态也有 Reconnect 条（B-05 修复后规格）** | ✅ T4/T4b/T5 |
| F021/F022 状态栏/Toast | 五态标签 + 三类 Toast + **断线 Toast（B-11）** | ✅ T4 |
| F023-F025 SSH/TOFU/多认证 | 行为模拟 + **B-12 确认式 TOFU 弹窗** + 无密钥场景（Permission 态） | ✅ T2/T12b/V1 组 |
| F026/F027 加密/持久化 | 保存 Toast 注明加密落盘；**修复后规格：加密失败拒绝保存（C-7）**（评审面板「后端事实」注明源码明文降级缺陷） | ✅（呈现核验）|
| F028 session_list | 评审面板「后端事实」如实注明后端已注册/前端未调用 | ✅（留档）|

### 2.2 占位功能 F029-F033（5 项，引导式空态呈现，不虚构）

| 功能 | V1 呈现 | 实测 |
|---|---|---|
| F029-F032 四占位视图 | **B-02 引导式空态**：旧文案事实保留 + 前往连接中心按钮 + 规划标注（虚线框「规划能力（不虚构）」指向 C-1） | ✅ T8/T9（SFTP 组含按钮跳转）|
| F033 命令面板 | **B-03 占位引导化**：占位声明 + 规划命令类别（按已实现动作归纳）+ 类别过滤演示 + 不虚构声明 | ✅ jsdom T10（开/聚焦/过滤/背景关闭）|

设置视图额外落地 **B-10**：引导按钮直达状态栏字号菜单（现有真实入口）。

### 2.3 规划功能 F034-F046（13 项，C 类留档不虚构）

评审面板「C 类待用户决策」清单逐条列出（C-1 汇总 13 项 + 5 占位排期；C-2..C-8 独立条目）；UI 中无任何虚构可用入口。✅

**功能覆盖结论：46/46，覆盖率 100%（28 保留 + 5 引导式占位 + 13 留档）。**

## 三、操作覆盖 / 五态 / DS 一致性

### 3.1 操作覆盖（所有按钮有效）

实测通过：活动栏 5 视图切换、侧栏折叠/拖宽、主机卡单击/双击/Connect/删除（危险确认两分支）、表单四字段输入与失焦校验、Connect、Save（查重+成功+按钮禁用恢复）、Enter 直连（新主机触发 TOFU）、TOFU 信任/中止两分支、Tab 点击/重命名（Enter）/关闭、Reconnect（closed 与 error 两态）、终端命令（ls/pwd/whoami/uptime/df -h/echo/exit/clear 及未知命令回退）、字号菜单、命令面板（开关/Esc/背景点击/输入过滤/自动聚焦）、评审面板 12 场景 + 11 快捷键模拟 + 10 页导航 + 3 演示按钮。**未发现无效按钮。**

### 3.2 五态覆盖

| 状态 | 场景/位置 | 实测 |
|---|---|---|
| Loading | 「加载中」场景：connecting 持续 + 黄点脉冲 + 状态栏 Connecting... | ✅ |
| Success | 默认场景：登录横幅 + 绿点 + 输入行 + `[status] connected` | ✅ T2r/T3 |
| Error | 认证/超时/被拒/DNS/不可达/MITM 六场景：红字 + 修复后文案 + Reconnect 条 | ✅ T6/T7/V1 组 |
| Empty | 空数据（无已存连接）/首次使用/PAGE009（无 Tab）/四占位视图 | ✅ T8 |
| Permission | 无可用密钥（密钥访问失败原文+源码事实注记）、TOFU 指纹确认框、密码掩码 | ✅ V1 组/T2 |

### 3.3 DS 一致性

- `:root` 全部令牌对齐 `docs/07_design_system/TOKEN.md`（26 项逐值核对：色板/间距/字号/布局/圆角【V1 建议组】）——✅。
- 组件与 `docs/07_design_system/COMPONENT.md` 记录表对应：HostCard/StatusDot(实现为全局 DOT 映射单源)/TabStrip/TerminalTab/AuthForm/KeyFingerprintConfirm/CommandPalette/Toast/EmptyState(引导式)/DangerConfirm 全部在原型中出现——✅。
- 终端配色语义（背景/前景/错误红/状态黄/提示符绿）与 docs/07_design_system/TOKEN.md §1.5 一致——✅。
- 零外链（无 https 资源引用）、图标全部内联 SVG（docs/07_design_system/ASSETS.md 14 枚）——✅。

## 四、缺陷记录与修复（测试负责人修复项）

| # | 缺陷描述 | 发现方式 | 修复 | 复验 |
|---|---|---|---|---|
| D1 | 首连判断 `isKnownHost` 逻辑写反（新主机不弹 TOFU、已存主机反而弹） | 代码走查（jsdom T12b 首轮异常） | 反转判断：未在已存连接中即 firstTime | ✅ T12b/T14 |
| D2 | `renderTabStrip` 创建 Tab 元素后未 `appendChild` 到条上（Tab 永不显示，状态点/重命名全部失效） | jsdom T13 首轮 `t0 undefined`；回溯解释了浏览器 T2 状态点断言失败 | forEach 末尾补 `strip.appendChild(d)` | ✅ 浏览器复验 `tabs=1 connected=true` + jsdom T13 全过 |
| D3 | 保存查重命中后 `submitting` 复位但未重渲染，Connect/Save 保持 disabled | jsdom T12 断言 | dup 分支补 `renderPanelContent()` | ✅ T12「按钮 disabled 恢复」 |
| D4 | PAGE009 空态外层容器无高度，引导内容不可见 | 代码走查 | 容器 `height:100%` | ✅ T8 |
| D5 | 认证失败提示的修复注记本身含 "Ctrl+R" 字样，导致 B-04 核验歧义 | 浏览器 T6 首轮 FAIL | 注记改写为「未注册快捷键死文案已移除」；grep 确认终端渲染路径零 "Ctrl+R" | ✅ V1 组「认证失败无 Ctrl+R」 |
| D6 | 「加载中」场景 toast 宣称 6 秒后成功，实际 outcome=hang 永不解决 | 代码走查 | toast 文案改为「connecting 持续保持（Loading 态演示）」 | ✅ |
| D7 | PAGE001 外壳无编号角标（铁律要求每页标注） | 验收清单核对 | `.app` 内补 PAGE001 角标（来源注明） | ✅ T1 |

修复后 `node --check` 复跑通过；jsdom 全量 25 + 10 断言 0 FAIL。

## 五、浏览器实测用例汇总

| 用例 | 内容 | 结果 |
|---|---|---|
| T1 | 主机卡×3、无下拉（B-01）、密码辅助（B-09）、PAGE001/002 角标 | ✅ |
| T2/T2r | Connect → TOFU 确认（出现/指纹/信任关闭）→ connecting → connected（横幅+输入行） | ✅ |
| T3 | uptime 回显 load average | ✅ |
| T4/T4b | 远端断开：`[status] closed` 行 + Reconnect 条（B-05）+ 断线 Toast（B-11）；重连后再断开 | ✅ |
| T5 | 切默认场景 Reconnect → connected | ✅ |
| T6 | 认证失败文案 + B-08 凭据指引（Ctrl+R 项经 D5 修复后由 V1 组复验） | ✅ |
| T7 | MITM 专案文案（B-13，含 known_hosts 处置指引） | ✅ |
| T8 | 空数据/首次使用 + PAGE009 引导按钮与角标 | ✅ |
| T9 | SFTP 引导式空态：旧文案保留 + 引导按钮 + 规划标注 + 跳回连接中心 | ✅ |

jsdom 组：T10 命令面板（5 项）、T11 危险确认（5 项）、T12 保存/查重/校验（6 项）、T12b Enter 直连 TOFU/中止（2 项）、T13 Tab 管理（6 项）、T14 单击回填/双击直连（3 项）、V1 补充组（10 项：错误映射、页面导航 10 项、B/C 清单计数、DS 令牌 26 项、零外链）——全部通过。

## 六、缺失列表

无。46 功能、10 页面、五态、13 项 B 类、DS 一致性五个维度逐项核对，未发现遗漏。

## 七、最终结论

# PASS ✅

- 功能覆盖：46/46（28 已实现保留 + 5 占位引导式 + 13 规划留档不虚构）
- 页面覆盖：10/10（每页带「编号+来源」角标；PAGE001-PAGE010 齐全）
- 操作覆盖：全部按钮有效
- 状态覆盖：五态齐全（Loading/Success/Error/Empty/Permission）
- B 类落地：13/13（B-01..B-13 逐项实测通过；其中 4 项为源码缺陷修复后规格呈现并注明：B-04/B-05/B-13 + C-7 呈现层）
- DS 一致性：令牌 26 项逐值对齐、组件与记录表对应、零外链
- `node --check`：通过（修复后复跑通过）
- 过程发现 7 项原型实现缺陷（D1-D7），全部修复并复验通过

### 非阻断备注

1. favicon.ico 404：仅 HTTP 服务访问时浏览器默认请求，file:// 直开不受影响，不处理（同 V0 备注）。
2. 浏览器宿主快捷键限制（Ctrl+W 关页签等）：评审面板提供等价模拟按钮；Tauri 实机无此限制（同 V0 备注）。
3. 测试环境浏览器被其他项目的并行自动化会话持续侵占（页签多次漂移至 open-iot-platform / Steering-BLE / RedisPilot 原型）：所有用例脚本带 URL 防御检查，漂移期间零误测；剩余用例改由 jsdom 无头环境完成，结论不受影响。
4. 源码级缺陷三项（session_close 泄漏 C-6、加密明文降级 C-7、session_list 未接 D-1）按「修复后规格/事实留档」方式呈现于 V1 与 P7 契约，修复实施属用户决策，不计入原型验收缺陷。
5. 截图 `termforge-v1-final.png`（Playwright 生成）已随本次验收存于 `docs/review/`（吸取 V0 截图未入库教训，见 product-review A-2）。

> 附：验收截图 `docs/review/termforge-v1-final.png`（默认场景：主机卡列表 + TOFU 信任后的 connected 终端 + 评审面板）。

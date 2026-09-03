# TermForge HTML 原型验收报告

> 验收人角色：测试负责人（不修改产品逻辑；仅当发现原型实现缺陷时修复并记录修复项）
> 验收日期：2026-09-02
> 验收对象：单文件可交互原型（勘误 A-1，2026-09-02 P4 审查：原 `prototype/app-prototype.html` 已归位为 `prototype/v0-old/app-prototype.html`，本报告结论对该 V0 原型负责；V1 新版原型验收见 `docs/09_test/V1_ACCEPTANCE.md`）
> 对比基准：`docs/01_reverse/REVERSE_ANALYSIS.md`（逆向报告）、`docs/02_product/PRD.md`（PRD）、`docs/02_product/PAGE_SPEC.md`（交互规格）、`docs/09_test/COVERAGE_CHECKLIST.md`（覆盖表）

---

## 一、验收方法

1. **静态校验**：提取原型内嵌 JS（36,309 字符）执行 `node --check` —— **语法通过**。
2. **浏览器实测**（加分项，已执行）：`python3 -m http.server 8742` 起临时服务，Playwright 驱动 Chrome 加载 `http://127.0.0.1:8742/app-prototype.html`，脚本化执行 20 组用例（T1-T20）并复验（R9/R11/T15r/T16r/T15br），覆盖连接主链路、失败链路、Tab 管理、场景切换、五态呈现。测试完成后服务已关闭。
3. **控制台检查**：全程唯一报错为 `favicon.ico 404`（单文件原型无 favicon，非功能缺陷，不影响 file:// 直开）。

## 二、五维覆盖检查

### 2.1 页面覆盖（10/10）

| 页面 | 基准 | 原型呈现 | 结论 |
|---|---|---|---|
| PAGE001 主工作台 | 逆向报告③ | 桌面窗口画框（titlebar 红黄绿 + "TermForge"）+ 活动栏/侧栏/Tab条/内容区/状态栏 | ✅ |
| PAGE002 连接中心 | 同上 | 已存连接下拉+列表+四字段表单+Connect/Save，角标 PAGE002·F004-F009 | ✅ |
| PAGE003 终端会话 | 同上 | 等宽终端、登录横幅、命令回显、状态点、PAGE003·F014-F025 角标 | ✅ |
| PAGE004 SFTP | 同上 | EmptyState 原文案（No active SFTP session…） | ✅ |
| PAGE005 隧道 | 同上 | EmptyState 原文案（No tunnels configured…） | ✅ |
| PAGE006 Runbook | 同上 | EmptyState 原文案（No runbooks yet…） | ✅ |
| PAGE007 设置 | 同上 | EmptyState 原文案（Settings…） | ✅ |
| PAGE008 命令面板 | 同上 | 占位面板+搜索框+placeholder 提示，Esc/背景关闭 | ✅ |
| PAGE009 终端空态 | 同上 | No active terminals + 引导文案，关闭全部 Tab 后出现 | ✅ |
| PAGE010 Toast | 同上 | 3 类、3s 自动消失、点击关闭、右下栈式 | ✅ |

### 2.2 功能覆盖（46/46）

- 已实现功能 F001-F028（旧项目行为基线）：全部有可交互对应物。
- 占位功能 F029-F033：按旧项目空状态如实呈现（不虚构功能）。
- 规划功能 F034-F046：评审面板"规划能力"清单逐条列出并标注"未实现"，符合"不脑补"原则。
- F028（session_list 后端有/前端未接）：评审面板"后端事实"区如实注明。

### 2.3 操作覆盖（所有按钮有效）

实测通过的交互清单：活动栏 5 视图切换、重复点击折叠、侧栏折叠按钮、拖宽（180-400px 钳制，实测 390px 生效）、表单输入/失焦校验、Connect、Save（查重+成功+刷新）、删除（confirm 取消/确认两分支）、下拉选择回填、列表单击回填、双击直连、Enter 直连、+ 新建、Tab 点击切换、Ctrl+1/2/3/9（含钳制）、Ctrl+Tab 双向循环、Tab 关闭（相邻激活）、Tab 双击重命名（Enter/Esc）、终端命令输入（ls/pwd/whoami/uptime/df -h/echo/exit/clear 及未知命令回退）、Reconnect、字号菜单 9 档、命令面板开关、Toast 点击关闭、评审面板 10 项页面导航 + 10 个场景 + 11 个快捷键模拟按钮。**未发现无效按钮。**

### 2.4 状态覆盖（五态）

| 状态 | 呈现位置 | 实测 |
|---|---|---|
| 加载（connecting） | 终端 "Connecting..." + Tab 黄点脉冲 + 状态栏 "Connecting... to x" | ✅ T2/T8 |
| 成功（connected） | Ubuntu 登录横幅 + 绿点 + "Connected" + 输入行 | ✅ T2b/T8b |
| 失败（error） | 红字 [error] + 友好文案 + Reconnect 条 | ✅ T15r |
| 空数据 | 无已存连接（隐藏下拉列表）/ PAGE009 / 四占位视图 | ✅ T17/T18/T12 |
| 异常 | 远端断开（[status] closed + Disconnected）、密钥变更、无密钥、超时等场景库 | ✅ T16r + 场景切换 |

### 2.5 异常覆盖

6 种 friendlyError 映射（认证/拒绝/超时/DNS/不可达/兜底）+ 主机密钥变更（含 MITM 原始错误附注）+ 无可用密钥 + 远端断开，共 10 个场景；每场景附后端原始错误说明。终端错误提示中的死文案 Ctrl+R 按旧项目事实保留并加注说明。✅

## 三、缺陷记录与修复（测试负责人修复项）

| # | 缺陷描述 | 发现方式 | 修复 | 复验 |
|---|---|---|---|---|
| D1 | 表单校验错误提示在重渲染后丢失（refreshErrors 先于 renderPanelContent 执行被覆盖） | T1（空表单提交错误数为 0） | 调整两函数调用顺序 | ✅ T1 复验 2 条错误（Host/Username），T4 端口错误正常 |
| D2 | Tab 重命名输入框未绑定 oninput，输入新标题按 Enter 后不生效 | T9（标题未变） | renderTabStrip 中为 rename input 增加 oninput 同步 state.editTitle | ✅ R9 标题更新为"生产机-01" |
| D3 | 命令面板背景点击不关闭（关闭逻辑挂在 mousedown，与点击语义不一致） | T11（背景点击关闭=false） | 改为 backdrop 直接 click 监听（对齐源码 CommandPalette 的 on:click） | ✅ R11 关闭=true |
| D4 | 切换场景会清空已开 Tab，导致"失败→切场景→Reconnect"评审流无法演示 | 测试设计评审 | applyScenario 不再清空 Tab | ✅ T15br：认证失败→切默认→Reconnect→Connected |

另有一次误报排查记录：T15/T16 首轮两个 FAIL 经定位为**测试脚本选择器问题**（多 Tab 时 `.term-body` 命中非激活面板），改用 `.terminal-pane.active .term-body` 后 T15r/T16r 均 OK，原型本身无缺陷。

## 四、浏览器实测用例汇总（T1-T20）

| 用例 | 内容 | 结果 |
|---|---|---|
| T1/T4 | 空表单校验（Host/Username 必填）、端口 70000 报 "Port must be 1-65535" | ✅ |
| T2 | 连接流程：connecting → connected，Tab 标题 username@host | ✅ |
| T3 | 终端命令 uptime 回显 load average | ✅ |
| T5/T6 | 保存查重冲突 Toast / 保存成功 Toast + 下拉刷新 | ✅ |
| T7 | 删除连接：confirm 可见、取消不变、确认删除 + 列表刷新 | ✅ |
| T8 | 双击已存连接直连 | ✅ |
| T9(修复后 R9) | Tab 重命名 | ✅ |
| T10 | 字号菜单 9 档（10-20px），选择后终端字体即时生效 | ✅ |
| T11(修复后 R11) | 命令面板：开/自动聚焦/背景点击关闭 | ✅ |
| T12 | SFTP/隧道/Runbook/设置四空态文案与源码一致 | ✅ |
| T13 | 重复点击活动栏当前视图 → 折叠/展开 | ✅ |
| T14 | 多 Tab：Ctrl+1 激活首格、Ctrl+9 钳制末位、Ctrl+Tab 双向循环、关中间 Tab 相邻激活 | ✅ |
| T15/T15r/T15br | 认证失败文案 + Reconnect + 切场景重连成功 | ✅ |
| T16/T16r | 远端断开：[status] closed 行 + Disconnected 状态 | ✅ |
| T17 | 空数据场景：下拉/列表消失、表单保留 | ✅ |
| T18 | 关全部 Tab → PAGE009 空态 + 状态栏 No active session | ✅ |
| T19 | PTY 尺寸模拟显示（66×36，随字号/窗口变化） | ✅ |
| T20 | 侧栏宽度设置生效 | ✅ |

## 五、缺失列表

无。页面、功能、操作、状态、异常五个维度对照四份基准文档逐项核对，未发现遗漏。

## 六、最终结论

# PASS ✅

- 页面覆盖：10/10（100%）
- 功能覆盖：46/46（100%，其中 13 项规划功能按"占位+清单如实标注"方式覆盖）
- 操作覆盖：全部按钮有效（含评审面板控件）
- 状态覆盖：五态齐全（加载/成功/失败/空/异常）
- 异常覆盖：10 场景 + 6 错误映射 + 权限/取消路径
- 过程中发现 4 项原型实现缺陷，全部修复并复验通过（D1-D4）

### 非阻断备注

1. favicon.ico 404：仅 HTTP 服务访问时的浏览器默认请求，file:// 直开不受影响，不处理。
2. 真实键盘快捷键在浏览器中受宿主限制（如 Ctrl+W 会关浏览器标签页），原型已提供评审面板快捷键模拟按钮作为等价入口；在 Tauri 实机中不存在此限制。
3. 旧项目已知缺陷（Ctrl+R 死文案、加密降级、session_close 泄漏）在原型中按事实保留并加注，属产品重开发待办，不计入原型验收缺陷。
4. 浏览器实测期间发现测试环境有一并行的其他自动化会话共用浏览器导致页签漂移，已通过重开页签规避，不影响验收结论（所有用例均在 TermForge 页面上执行，带页面 URL 防御性检查）。

> 附：验收过程截图 `termforge-prototype-final.png`（勘误 A-2，2026-09-02 P4 审查复核：该截图未随仓库留存，全仓检索无此文件，此处仅作历史记录说明）。

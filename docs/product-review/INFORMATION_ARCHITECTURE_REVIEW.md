# TermForge 信息架构评审（INFORMATION ARCHITECTURE REVIEW）

> 评审依据：《AI 产品重构逻辑评审规范 v1.0》· 2026-09-03
> 输入文档清单：docs/01_reverse/REVERSE_ANALYSIS.md（P1）、docs/02_product/PRD.md（P2）、docs/02_product/PAGE_SPEC.md（P3）、docs/06_review/PRODUCT_REVIEW.md（P4）；源码抽查 src-ui/src/App.svelte、src-ui/src/components/layout/{ActivityBar,SidePanel,TabStrip,StatusBar}.svelte、src-ui/src/components/{ConnectionForm,TerminalTab}.svelte、src-ui/src/components/primitives/{CommandPalette,EmptyState,ToastContainer}.svelte。
> 铁律遵守：只评审不修改；所有「当前设计」均附源码依据；无法证实处标【未知】。

---

## 一、导航层级图（代码实测）

```
TermForge 主窗口（单窗口，无路由库；视图切换 = activeView 字符串）
│
├─ [一级导航] ActivityBar（48px，5 个视图入口）
│   ├─ Connections ──▶ SidePanel slot "connections"（唯一有内容的侧栏视图）
│   ├─ SFTP ──────────▶ SidePanel EmptyState（占位）
│   ├─ Tunnel ────────▶ SidePanel EmptyState（占位）
│   ├─ Runbook ───────▶ SidePanel EmptyState（占位）
│   └─ Settings ──────▶ SidePanel EmptyState（占位）
│
├─ [工作区] TabStrip + 终端容器（{#each tabs}，display 切换）
│   ├─ Tab（每连接一个）──▶ TerminalTab（xterm + Reconnect 条）
│   └─ 无 Tab ──────────▶ EmptyState「No active terminals」（PAGE009）
│
├─ [状态] StatusBar（左：当前 Tab 状态；右：UTF-8 + 字号菜单）
│
└─ [浮层] CommandPalette（Ctrl+Shift+P，占位）；Toast（全局反馈）

跳转关系（page-spec §1 PAGE001 汇总，经源码核实一致）：
ActivityBar → 5 视图；ConnectionForm Connect/双击 → 新 Tab；
Ctrl+T/+按钮 → connections 视图；关闭全部 Tab → PAGE009。
无「返回」「历史」概念（单层视图模型，桌面工具合理）。
```

关键代码依据：App.svelte L43（activeView）、L254-264（handleViewChange：同视图点击=折叠切换）；SidePanel.svelte L146-156（按 activeView 渲染 slot/EmptyState）。

---

## 二、逐页分类归属表

分类口径（规范 §7）：功能页 / 列表页 / 详情页 / 表单页 / 设置页 / 反馈层 / 空态引导页。

| 页面（P1 编号） | 归属分类 | 判定依据 | 职责单一性 |
|---|---|---|---|
| PAGE001 应用主工作台 | 外壳/容器页 | App.svelte 全文 | 单一（布局+快捷键） |
| PAGE002 连接中心 | 列表页 + 表单页（**混合**） | ConnectionForm 同时渲染已存列表与新建表单（L141-204） | 混合（见 IA-03） |
| PAGE003 终端会话页 | 功能页（核心工作区） | TerminalTab.svelte | 单一 |
| PAGE004 SFTP 视图 | 空态引导页（占位） | SidePanel L149 | 单一（空） |
| PAGE005 隧道视图 | 空态引导页（占位） | SidePanel L151 | 单一（空） |
| PAGE006 Runbook 视图 | 空态引导页（占位） | SidePanel L153 | 单一（空） |
| PAGE007 设置视图 | 设置页（**空壳**） | SidePanel L155 | **名不副实**（见 IA-02） |
| PAGE008 命令面板 | 快捷入口浮层（占位） | CommandPalette.svelte | 单一（空） |
| PAGE009 终端区空态 | 空态引导页 | App.svelte L297-302 | 单一 |
| PAGE010 Toast | 反馈层 | ToastContainer.svelte | 单一 |

结构性观察：**10 个页面中 4 个是占位空态（PAGE004-007）、1 个占位浮层（PAGE008）——IA 骨架按终局设计（五视图工作台）搭建，内容兑现度 50%**。

---

## 三、可预测性抽查（7 例，≥5 要求达标）

> 抽查方法：以「用户心智预期 → 实际位置/行为」对照，标注命中/偏移/不可达。

| # | 用户意图 | 用户预期位置 | 实际位置/行为 | 依据 | 判定 |
|---|---|---|---|---|---|
| 1 | 调整终端字号 | Settings 视图 | 状态栏右侧字号菜单（运行时生效不持久） | StatusBar.svelte L84-105 vs SidePanel L155 空态 | **偏移** |
| 2 | 修改已存连接的密码 | 连接列表项或表单的编辑入口 | 不可达：无编辑按钮；Save 生成新 UUID 必被查重拦 | ConnectionForm L67-77 | **不可达** |
| 3 | 断线后重连 | 终端区重连按钮 | error 态有按钮；closed 态无；运行时读错误态连状态都不变 | TerminalTab L132、L188 | **不一致** |
| 4 | 删除连接 | 列表项上的删除控件 | hover 才显示 ×（键盘/触控不可发现）；下拉入口无删除 | ConnectionForm L153-170 | **弱可发现** |
| 5 | 用 SFTP 传文件 | 活动栏 SFTP | 入口可达，内容为空态占位 | SidePanel L149 | 可达不兑现 |
| 6 | 新开一个终端 Tab | TabStrip「+」= 开新空 Tab | 实际是「跳转到连接中心」引导，不创建 Tab | TabStrip tooltip "New connection (Ctrl+T)"；App.svelte L196-199 | **语义偏移** |
| 7 | 取消正在进行的连接 | 连接中可见的取消按钮 | 不可达（15s 超时兜底） | TerminalTab L143-150 | **不可达** |

抽查结论：7 例中 2 例可达（其中 1 例不兑现）、2 例不可达、3 例偏移/不一致——**可预测性约 29%，低于可用性基线**。其中 #1、#3、#6 为 P4 已记录问题（P-02/FL-02/FL-08），#2、#7 为 P4 已记录（F-02/FL-06），本表将它们置于统一可预测性视角下量化。

---

## 四、信息架构分层合理性

| 维度 | 现状 | 评价 |
|---|---|---|
| 层级深度 | 2 层（视图 → 页面内容），浮层独立 | 合理，桌面工具最佳区间 |
| 导航宽度 | 活动栏 5 入口 | 宽度合理，但 3 个入口无内容支撑（IA-01） |
| 主工作区模型 | Tab 制（每连接一 Tab） | 与目标用户（P1 多会话并行）匹配 |
| 全局命令入口 | 命令面板占位 + 9 组快捷键 | 快捷键与菜单/面板间无冗余（面板无命令）；快捷键在输入/终端聚焦时失效（P4 F-07/D-2），一致地失效=一致 |
| 信息冗余 | 已存连接双入口（下拉+列表，P4 F-01/B-01） | 冗余 |
| 状态可见性 | 状态点三处呈现（Tab 条/状态栏/终端内） | 合理冗余（不同粒度） |
| 检索能力 | 连接列表无搜索/过滤/分组 | 超过 ~10 条后线性扫描（IA-04） |

---

## 五、问题清单

> 格式：当前设计 / 问题 / 影响 / 建议方向。分级沿用 A/B/C/D。与 P4 重叠条目交叉引用。

### IA-01【C】三个常驻导航入口长期无内容兑现（SFTP/Tunnel/Runbook）
- **当前设计**：活动栏 5 视图常驻（ActivityBar.svelte），其中 3 个为 EmptyState 占位（SidePanel L149-153）。
- **问题**：IA 骨架承诺「五模块工作台」，交付「SSH 客户端 + 设置空壳」；导航承诺与内容兑现度长期背离。P4 P-01/B-02 在原型层做了引导式空态（呈现优化），但「未实现视图是否常驻一级导航」是产品结构决策，P4 归入 C-1 未单独立条。
- **影响**：新用户前 5 分钟内点开占位视图即建立「半成品」第一印象；五等分的导航权重虚高。
- **建议方向**：重开发时二选一：(a) 未实现视图不入一级导航（发布时只放 Connections，随功能落地逐步放开）；(b) 保留但引导式空态 + 明确「规划中」标识。属用户决策（与 PL-01 功能取舍联动）。

### IA-02【B】设置能力分裂：设置页空壳，唯一设置藏在状态栏（P4 P-02/B-10+C-3 交叉引用）
- **当前设计**：字号菜单在 StatusBar.svelte L84-105（运行时、不持久化）；Settings 视图为 EmptyState（SidePanel L155）。
- **补充视角**：字号不持久化（F034 未实现）使状态栏入口的价值进一步弱化——一个「不记得住」的设置无论放哪都构不成设置心智。
- **建议方向**：设置页应成为设置项的唯一归口（含字号迁移+持久化）；状态栏保留快捷调节。落位与持久化范围属 C-3。

### IA-03【B】连接中心双入口冗余且能力不对等（P4 F-01/B-01 交叉引用）
- **当前设计**：同一 savedConnections 渲染为 select 下拉（仅回填）与列表（回填/双击直连/删除）（ConnectionForm L141-174）。
- **补充视角**：IA 层面这是「同一数据两种呈现不同能力」的规则破坏——用户无法预测任何入口的完整能力集。
- **建议方向**：与 P4 B-01 一致（收敛单一列表），重开发时列表项能力应完整（选中/直连/编辑/删除）。

### IA-04【B】连接列表无检索/过滤，规模可扩展性缺口（P4 未单独立条）
- **当前设计**：`.saved-list` 全量线性渲染（ConnectionForm L152-172）；无搜索框、无分组、无最近使用排序（F036 全部未实现）。
- **问题**：P1 画像（每天连多台机器）与 PRD P3（统一管理连接）的核心诉求下，10-50 条连接时侧栏 260px 内的线性列表不可用。
- **影响**：高频用户的核心资产（连接库）检索成本随规模线性上升。
- **建议方向**：重构基线纳入轻量前端过滤框（输入即筛 name/host/username）；完整分组/标签/导入导出维持 P2 规划（PL-01 取舍）。

### IA-05【D】命令面板占位无内容（P4 P-03/B-03 交叉引用）
- 引导化呈现已处理；真实命令集归 F033/C-1。

### IA-06【D】TabStrip 溢出无视觉指示（P4 P-05/D-3 交叉引用）
- 多 Tab 场景；观察。

### IA-07【B】「新建 Tab」入口语义偏移（P4 FL-08/B-11 交叉引用）
- `+`/Ctrl+T 实为「去连接中心」（App.svelte L196-199）；可预测性抽查 #6 命中。重开发时统一为「新建连接」语义并允许未来真正的「新 Tab」入口共存。

### IA-08【D】快捷键在输入/终端聚焦时整体失效（P4 F-07/D-2 交叉引用）
- IA 视角补充：这使得「快捷键」在产品最常用的两个上下文（终端、表单）中不可用，命令面板（同样受限于 L69 输入守卫前的 Escape 特例）成为唯一全局入口却又无命令——全局导航可达性在最需要时最低。维持 D（透传惯例合理），但与 F033 联动决策。

---

## 六、小结

| 分级 | 数量 | 编号 |
|---|---|---|
| B | 4 | IA-02、IA-03、IA-04、IA-07 |
| C | 1 | IA-01 |
| D | 3 | IA-05、IA-06、IA-08 |

IA 层最关键结论：**骨架（层级/宽度/Tab 模型）本身是好的 VS Code 范式移植；问题集中在「承诺-兑现差」（IA-01）与「入口能力不对等」（IA-03/IA-07）。重开发时 IA 决策只有两个：占位视图的导航策略（C）、连接中心的单一入口+检索（B）**。

# TermForge 状态评审（STATE REVIEW）

> 评审依据：《AI 产品重构逻辑评审规范 v1.0》· 2026-09-03
> 输入文档清单：docs/01_reverse/REVERSE_ANALYSIS.md（P1）、docs/02_product/PRD.md（P2）、docs/02_product/PAGE_SPEC.md（P3）、docs/06_review/PRODUCT_REVIEW.md（P4）；源码抽查 src-ui/src/lib/api.ts、src-ui/src/lib/types.ts、src-ui/src/App.svelte、src-ui/src/components/TerminalTab.svelte、src-ui/src/components/layout/TabStrip.svelte、src-ui/src/components/layout/StatusBar.svelte、src-ui/src/components/ConnectionForm.svelte、src-tauri/src/core/session_manager.rs、src-tauri/src/core/ssh/client.rs、src-tauri/src/models/events.rs（经 P1 §⑦ 转引核对）。
> 铁律遵守：只评审不修改；所有「当前设计」均附源码依据；无法证实处标【未知】。

---

## 一、状态全景与状态机文本图

产品内共存在 **4 套独立状态体系**，彼此靠事件松耦合：

1. **前端 TabStatus（五态）**——`src-ui/src/lib/types.ts`：`idle | connecting | connected | closed | error`；由 TerminalTab 内部 `setStatus` 驱动（TerminalTab.svelte L28-32），向上 dispatch 同步 TabStrip 状态点与状态栏。
2. **后端 SessionHandle.status**——session_manager.rs L18-23；open_ssh 成功时写死 `"connected"`（L77），此后**无任何更新代码路径**。
3. **事件流状态（app_event / terminal:status）**——client.rs IO 线程 emit：`connected`（open_ssh L83）、`closed`（EOF L216-224 / 主动关 L296-303）、`error`（读错误 L242-253 / 写错误 L266-276 / resize 错误 L281-289）。
4. **页面局部状态**——activeView×5、sidePanelCollapsed、fontSize、表单 errors/touched/submitting、selectedConnId、命令面板 visible、Toast entering/leaving。

### 1.1 前端 Tab 状态机（代码实测转换）

```
                 ┌────────────────────────────────────────────┐
                 │                                            │
   createTab     ▼          connect()            session_open │
  ┌──────┐  ┌────────────┐  15s内成功  ┌───────────┐           │
  │ idle │─▶│ connecting │──────────▶│ connected │           │
  └──────┘  └─────┬──────┘           └─────┬─────┘           │
     （创建即连）    │ 15s超时/连接失败         │ 收到事件 closed │
                    ▼                        ▼ (仅此一条路径)  │
                ┌───────┐               ┌────────┐            │
                │ error │◀──────────────│ closed │            │
                └───┬───┘   （无此转换） └────┬───┘            │
                    │ reconnect()            │ reconnect()    │
                    │（能力在 L179-183）       │（能力同）       │
                    └───────────┬─────────────┘                                │
                                ▼          回到 connecting                     │
```

**代码核实的关键事实**：
- `closed → connecting`：reconnect() 允许 `error || closed` 两态重连（TerminalTab.svelte L179-183），但 Reconnect 按钮仅在 `{#if status === 'error'}` 渲染（L188-192）——closed 态转换存在、触发器缺失（P4 FL-02 已记录）。
- **运行时 error 事件不产生任何状态转换**：事件处理只写 `if (ev.status === 'closed') setStatus('closed')`（L132）；后端 emit 的 `error` 状态仅被打印为 `[status] error: ...` 文本行（L131）。api.ts L7 的类型联合 `terminal:status.status: 'connected' | 'reconnecting' | 'closed'` **不含 'error'**——前端类型层就把后端真实会发出的值排除了。

### 1.2 后端会话状态机（代码实测）

```
 open_ssh 成功                IO 线程事件                 close() 命令
┌────────────┐  EOF     ┌──────────────┐  emit closed ┌────────────┐
│ HashMap 中 │────────▶│ IO 线程退出   │─────────────▶│ 事件发出    │
│ status=    │  读错误  │ （emit error │              │ 但 HashMap │
│"connected" │────────▶│  但不回写    │              │ 仍留句柄†  │
│ （写后不再  │          │  status）    │              └────────────┘
│  更新）     │          └──────────────┘                remove+close
└────────────┘                                         （仅用户关 Tab）
```

† session_manager.rs L111-120：close() 是唯一移除句柄的路径；IO 线程退出（EOF/读错误）后无回调清理。后果：(a) `session_list` 对已死会话仍报 `connected`；(b) 用户不关 Tab 则句柄与 OS 线程资源占用至应用退出（EOF/读错误场景线程已退出，仅句柄残留；写错误场景连接仍在但状态已陈旧）。

### 1.3 状态双轨对照表（前端 TabStatus vs 后端事实）

| 场景 | 后端事实 | 后端 SessionHandle.status | 事件流 | 前端 TabStatus | 一致性 |
|---|---|---|---|---|---|
| 连接建立 | 已连接 | connected | connected | connected | 一致 |
| 远端 EOF | IO 线程退出，会话死 | **connected（陈旧）** | closed | closed | 后端两处不一致 |
| 读错误 | IO 线程退出，会话死 | **connected（陈旧）** | **error** | **connected（不变）** | 三方不一致（ST-01） |
| 写错误 | 连接保持可用 | connected | error（仅提示） | connected（不变） | 状态缺失但行为可辩护 |
| 15s 前端超时、后端 18s 成功 | 会话已注册 | connected | connected（被过滤） | **error** | 状态分裂（ST-03） |
| 用户关 Tab、close 失败 | 句柄泄漏 | connected | — | Tab 已移除 | 泄漏（P4 FL-04/C-6） |

---

## 二、逐状态必要性审查（规范 §10：必要 / 过多 / 冲突 / 重复）

| 状态 | 必要性 | 审查结论 |
|---|---|---|
| TabStatus.idle | 存疑（过细） | createTab 到 connect() 之间的瞬态（App.svelte L172 创建即 idle，TerminalTab onMount 立即 connect）；实际不可观测（connecting 几乎同时出现）。TabStrip/StatusBar 却为它定义了灰点与 "Idle" 文案。保留无害，重构可合并入 connecting。 |
| TabStatus.connecting | 必要 | 15s 窗口的可视反馈（黄点脉冲 + "Connecting..."）。 |
| TabStatus.connected | 必要 | 核心可用态。 |
| TabStatus.closed | 必要 | 远端断开与用户感知需要区分「主动关/被动断」。 |
| TabStatus.error | 必要 | 但覆盖不足（ST-01）：设计上应承接运行时错误，实际只承接连接建立失败。 |
| api.ts 'reconnecting' | 冗余（预留） | 后端永不 emit（ST-05）。 |
| SessionHandle.status | 冗余且失真 | 写后不更新（ST-04）；session_list 前端又从未调用（P4 F-04/D-1）——双重闲置。 |
| activeView / collapsed / fontSize | 必要 | 外壳基本状态。 |
| 表单 errors/touched/submitting | 必要 | 但 submitting 仅覆盖 Save（ST-07）。 |
| Toast entering/leaving | 必要 | 动画瞬态。 |
| 重复定义：statusColors | 重复 | TabStrip.svelte L26-32 与 StatusBar.svelte L18-24 各一份（P4 4.1 已记 StatusDot 抽取建议）；statusLabels 仅 StatusBar 一份。 |

---

## 三、问题清单

> 格式：当前设计 / 问题 / 影响 / 建议方向。分级沿用 A/B/C/D。

### ST-01【B】运行时 error 事件不映射前端状态：死会话显示绿点、输入静默丢弃
- **当前设计**：后端读错误 emit `terminal:status{status:"error"}` 后 IO 线程退出（client.rs L242-253）；前端 api.ts L7 类型联合不含 'error'，TerminalTab.svelte L130-133 仅对 closed 变更状态。
- **问题**：状态机缺 `connected → error` 转换的事件入口；类型层与实现层双重缺口（P4 未覆盖——P4 FL-02 只发现 closed 态无按钮）。
- **影响**：网络闪断读错误后：Tab 绿点、状态栏 "Connected"、无 Reconnect 按钮；用户键入经 sessionSend 到已断的 mpsc 通道，send 失败被 `.catch(() => {})` 静默吞掉（TerminalTab L84）——终端看起来活着实则死亡，用户唯一出路是关 Tab。
- **建议方向**：api.ts 类型补 'error'；TerminalTab 事件分支对 error 调 `setStatus('error')`（写错误可仅提示不断态，读错误必须断态）；重开发规格中把该转换写入验收。

### ST-02【B】closed 态的重连转换存在但无 UI 触发器
- **当前设计**：reconnect() 允许 closed（TerminalTab L179-183）；Reconnect 条仅 error 渲染（L188）。
- **问题/影响**：远端正常断开后无重连入口，用户关 Tab 重来。
- **建议方向**：与 P4 B-05 一致（closed 态渲染 Reconnect 条），本册从状态机完整性角度将其列为必须补齐的转换触发器，交叉引用不重复展开。

### ST-03【B】前端 15s 超时与后端成功并发：状态分裂 + 孤儿会话
- **当前设计**：前端 Promise.race 15s 超时置 error（TerminalTab L143-150）；后端 spawn_blocking 继续执行，TCP read timeout 30s（client.rs L125），15-30s 间完成后即注册进 HashMap 并 emit connected（session_manager.rs L70-84）——此时前端 capturedSessionId 为空，事件被过滤（TerminalTab L124-125），sessionId 赋值代码永不执行（race 已 reject）。
- **问题**：超时不是取消协议，只是前端放弃等待；UI=error 与后端=connected 长期并存；用户点 Reconnect 再开新会话，旧会话无人 close。
- **影响**：慢网络下会话泄漏累积（连接数、远端 sshd 进程、本机句柄）；「Connection timed out」提示与「实际已连上」的事实矛盾，误导用户调网络。
- **建议方向**：重开发时定义 open 的取消语义：前端超时后调用 session_close（按 pending 句柄幂等清理），或后端 open_ssh 支持取消令牌；P4 C-6 的「close 失败强制回收」规格应与本项合并为统一的会话生命周期规格。

### ST-04【B】后端 SessionHandle.status 写后即陈旧；远端断开后句柄残留
- **当前设计**：见 §1.2——status 无更新路径；句柄唯一移除点是 close()。
- **问题**：状态必要性与真实性双缺——既然 session_list 的数据失真且前端不用（P4 D-1），该字段当前价值为零却维持维护成本。
- **影响**：session_list 一旦接入（命令面板/状态总览）即输出错误数据；泄漏句柄在应用生命周期内累积。
- **建议方向**：IO 线程退出时回调 SessionManager 更新状态并标记待回收；或删除该字段、list 改为输出线程存活探测。

### ST-05【C】'reconnecting' 预留枚举无语义
- **当前设计**：api.ts L7 含 'reconnecting'；后端无 emit 点；F037 自动重连未实现。
- **问题**：类型承诺不存在的状态，误导重开发者以为有自动重连骨架。
- **影响**：低；认知噪音。
- **建议方向**：随 13 项功能取舍（PL-01）一并定：做 F037 则定义其转换（closed→reconnecting→connected/error），不做则删枚举。

### ST-06【D】状态映射两处重复定义
- **当前设计**：TabStrip.svelte L26-41 与 StatusBar.svelte L18-32。
- **建议方向**：P4 4.1 已建议抽取 StatusDot 组件，观察随重构落地；不重复立项。

### ST-07【D】Connect 无防重复提交
- **当前设计**：submitting 仅在 handleSave 置位（ConnectionForm L64、L89）；handleConnect 只读不写（L45）。
- **问题**：快速双击 Connect 会开两个同目标 Tab（「每连接一个 Tab」模型下语义合法但非用户意图）。
- **建议方向**：观察；重构时给 Connect 加 300ms 防抖或 submitting 共用即可。

---

## 四、缺失状态汇总（规范 §10「缺失概念」）

1. **连接取消态**：connecting 无法中止（P4 FL-06/D-5 交叉引用）——15s 兜底掩盖了「用户想立刻放弃」的状态缺失。
2. **密码不可解态**（DS-01）：存储层有此事实、产品层无此概念，用户无法区分「无密码」与「密码失效」。
3. **首次连接/指纹确认态**：TOFU 自动信任（client.rs L65-74），无「待确认」状态（P4 FL-09/B-12 的状态层根源）。
4. **应用级「有会话在后台断线」态**：非激活 Tab 断线仅状态点变灰，无全局信号（P4 FL-11/B-05 附带已覆盖呈现层）。

---

## 五、小结

| 分级 | 数量 | 编号 |
|---|---|---|
| B | 4 | ST-01、ST-02、ST-03、ST-04 |
| C | 1 | ST-05 |
| D | 2 | ST-06、ST-07 |

状态层最关键结论：**五态状态机设计本身合理，裂缝全部在「运行时错误」与「前后端双轨同步」两处——ST-01（error 事件无映射）与 ST-03（超时非取消）是 P4 完全未覆盖的两个缺口，重开发规格必须补齐，否则「状态可视」这一核心价值主张（P1 §① 核心价值 3）在异常场景下失效**。

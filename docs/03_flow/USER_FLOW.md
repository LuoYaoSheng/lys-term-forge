# TermForge 用户旅程（USER FLOW）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 内容来源：旅程步骤与判定分支全部取自 `docs/01_reverse/REVERSE_ANALYSIS.md` §⑥ 用户流程（Mermaid 四图）与 §④ 页面详细分析；画像/场景口径见 `docs/02_product/PRD.md` §2-3；异常恢复缺口引 `docs/product-review/USER_FLOW_REVIEW.md`（UF 编号）。流程图为源图重组，无新增行为。

---

## 旅程 1：P1 日常登录排查（场景 S1）——主链路（正常路径）

用户：高频 SSH 开发者，每天连多台机器。目标：最快进入远端 shell 干活。

```mermaid
flowchart TD
    A[启动应用] --> B[App onMount 调 connection_list]
    B --> C[侧栏渲染已存连接列表]
    C --> D{用户操作}
    D -->|双击已存连接| F[回填并直接 Connect 加速路径]
    D -->|选择已存连接单击/Enter| G[回填表单]
    G --> H[点击 Connect]
    D -->|填写表单| E[Connect 校验]
    E -->|校验失败| ERR[字段下方红字提示] --> D
    E -->|校验通过| I[创建 Tab 状态 idle]
    F --> I
    H --> I
    I --> J["状态 connecting<br/>终端显示 Connecting... 黄点脉冲"]
    J --> K["session_open: TCP → 握手 → 主机密钥验证 → 认证 → PTY+shell"]
    K -->|15 秒内成功| L[返回 session_id]
    L --> M[状态 connected 清屏进入 shell]
    M --> N[终端输入 session_send]
    N --> O["IO 线程读取 app_event terminal:data"]
    O --> P[xterm write 实时渲染输出]
    P --> N
    N --> Q["结束: Ctrl+W / Tab × → session_close → 移除 Tab"]
```

**旅程要点**（来源：REVERSE_ANALYSIS §⑥ 流程 1；USER_FLOW_REVIEW.md F1）：
- 双击直连为主链路 2 步可用（回填+立即 dispatch connect，ConnectionForm L122-130），是产品当前最有价值的部分。
- 双击直连可发现性弱（仅 hover title 提示）——P4 F-06/B-07，V1 原型已按主机卡+Connect 快连按钮呈现。

## 旅程 2：P1/P2 首次连接新机器并保存（场景 S2）

用户目标：第一次连一台新机器，验证可连后留档复用。

```mermaid
flowchart TD
    A[打开连接中心 Ctrl+T 或 Ctrl+Shift+N 聚焦 Host] --> B[填写 host/port/username/password]
    B --> C[点击 Connect]
    C --> D{校验}
    D -->|失败| E[红字提示 终止] --> B
    D -->|通过| F[创建 Tab connecting]
    F --> G["session_open: 首连 TOFU 自动记录指纹到 known_hosts（用户无感知）"]
    G -->|成功| H[connected 进入 shell 使用]
    G -->|失败| I[error 态 + 6 种友好文案 + Reconnect]
    H --> J[回到表单重新填写同样参数]
    J --> K[点击 Save]
    K --> L{客户端查重: name 或 host+port+username}
    L -->|重复| M[Toast already exists]
    L -->|通过| N["connection_save: AES-256-GCM 加密密码<br/>写 connections.json 0600"]
    N --> O[Toast saved 刷新列表]
```

**旅程要点**（来源：REVERSE_ANALYSIS §⑥ 流程 1 Save 分支 + 流程 4；USER_FLOW_REVIEW.md F2/UF-08）：
- 断点：连接成功后想保存，必须重新填写一遍刚输入的参数（Connect 不落盘、无保存引导）——现路径 8+ 步（UF-08，D 类观察）。
- 首连 TOFU 自动信任、用户全程无感知（client.rs L65-74）——PM-03/B-12 已建议升级为确认式 TOFU。

## 旅程 3：P1 连接失败处置（场景 S3）——异常路径

用户目标：连不上时快速定位原因并恢复。

```mermaid
flowchart TD
    A[用户点 Connect] --> B[状态 connecting]
    B --> C{"15 秒内 session_open 结果"}
    C -->|Connection refused| D["friendlyError: check host and port"]
    C -->|Authentication failed| E["friendlyError: check username and password"]
    C -->|timeout / 15s 竞速超时| F[friendlyError: Connection timed out]
    C -->|主机密钥不匹配| G["错误信息含 MITM 警告 → UI 兜底显示 Connection failed"]
    C -->|DNS 失败| H[Host not found]
    D & E & F & G & H --> I[状态 error]
    I --> J["终端红字 error + 重试提示文案<br/>注: Ctrl+R 提示为死文案 实际未注册该快捷键"]
    J --> K[显示 Reconnect 按钮]
    K --> L{用户操作}
    L -->|点 Reconnect| M["重新走 connect 流程<br/>注意: 使用 Tab 创建时的凭据快照"]
    L -->|修改表单凭据后再点 Reconnect| N["无效——快照未变 须关 Tab 重建<br/>UF-01 三重锁死"]
    L -->|关闭 Tab| O[session_close 移除 Tab]
    L -->|放置不管| P[保持 error 态]
```

**旅程要点**（来源：REVERSE_ANALYSIS §⑥ 流程 2；USER_FLOW_REVIEW.md F4/UF-01/UF-06）：
- 错误提示 "Press Ctrl+R..." 为死文案（全局快捷键表未注册 Ctrl+R）——P4 FL-01/B-04。
- 认证失败修正路径断裂：Reconnect 用 Tab 创建时的 connection 快照，改表单对已建 Tab 无效——UF-01（B 类，重开发须闭环）。
- 主机密钥变更（MITM）被 friendlyError 兜底吞掉——P4 FL-10/B-13，V1 已增第七条专案映射。

## 旅程 4：P1/P2 会话中断处置（场景 S4）——边界路径

用户目标：远端断开或网络错误后，尽快恢复会话。

```mermaid
flowchart TD
    A[会话 connected 正常使用] --> B{中断类型}
    B -->|远端关闭 EOF| C["app_event terminal:status closed"]
    C --> D["终端 writeln status closed<br/>状态置 closed 灰点"]
    B -->|网络读错误| E["app_event terminal:status error Read error"]
    E --> F["终端显示一行错误文本<br/>但状态不变仍 connected 绿点 IO 线程已退出<br/>后续输入静默丢弃 ST-01/UF-02"]
    B -->|写错误| G["terminal:status error Write error<br/>报错但连接保持"]
    D --> H{恢复路径}
    H -->|点 Reconnect| I[重连]
    H -->|无重连按钮| J["源码缺陷: closed 态不渲染 Reconnect 条<br/>只能关 Tab 重来 FL-02/B-05"]
    F --> K["无提示 无按钮 无恢复<br/>唯一出路关 Tab UF-02"]
    B -->|用户主动 Ctrl+W| L{Tab 有 sessionId?}
    L -->|是| M[调 session_close]
    M -->|成功| N["IO 线程收 Close 命令 → channel.close → 推 closed 事件 → 线程退出"]
    M -->|失败| O["Toast Failed to close session<br/>Tab 仍移除 后端句柄泄漏 FL-04/C-6"]
    L -->|否| P[直接移除 Tab]
    N & O & P --> Q[激活相邻 Tab 或显示 No active terminals]
```

**旅程要点**（来源：REVERSE_ANALYSIS §⑥ 流程 3；STATE_REVIEW.md ST-01/ST-02；USER_FLOW_REVIEW.md F5/UF-02）：
- closed 态重连能力存在但按钮未渲染（能力在、入口漏）——FL-02/B-05。
- 运行时读错误不进入状态机：死会话显示绿点、输入被 `.catch(() => {})` 静默吞——ST-01/UF-02/PL-05，P4 未覆盖，重开发规格必须补齐。
- 非激活 Tab 断线无全局信号（无 Toast）——FL-11/B-05 附带。

## 旅程 5：P3 凭据资产沉淀（保存/删除/跨机迁移）——数据生命周期路径

用户目标：把连接配置作为长期资产管理。

```mermaid
flowchart TD
    A[填表点 Save] --> B{校验}
    B -->|失败| C[红字提示 终止]
    B -->|通过| D{客户端查重: name 或 host+port+username}
    D -->|重复| E[Toast already exists error]
    D -->|不重复| F[connection_save 加密落盘]
    F -->|加密失败| G["warn 后明文落盘（缺陷<br/>触发条件近死且明文读不回 DS-02）"]
    F -->|成功| H[Toast saved 刷新列表]
    F -->|IO 失败| I[Toast Failed to save connection]
    J[列表项 hover × 删除] --> K[原生 confirm 确认]
    K -->|取消| L[无操作]
    K -->|确认| M[connection_delete → Toast → 刷新]
    N[换机器/改用户名/改主机名] --> O["机器绑定密钥失配<br/>全部已存密码解密失败"]
    O --> P["list 时静默置 password=None 仅后端 warn<br/>前端表现为该连接无密码 DS-01"]
    P --> Q["双击直连走密钥探测 → 大概率认证失败<br/>提示 check username and password 误导排查"]
    Q --> R["无编辑入口 查重拦截<br/>用户被迫删除重建 UF-01+DS-01 叠加"]
```

**旅程要点**（来源：REVERSE_ANALYSIS §⑥ 流程 4；DATA_STORAGE_REVIEW.md DS-01/DS-02；USER_FLOW_REVIEW.md 三类异常核查表）：
- 正常保存/删除链路完整（F6 通过五要素核查）。
- 高频数据风险在读路径不在写路径：解密失败静默吞密码（DS-01，B 类）优先级高于加密失败明文降级（DS-02，触发近死，PL-04 校准）。
- 用户数据零出口（无导出/备份/迁移）+ 密码跨机失效叠加 = 换机等于从零重建（DS-06）。

## 旅程小结（来源：USER_FLOW_REVIEW.md §二/§五）

| 旅程 | 五要素核查 | 结论 |
|---|---|---|
| 1 日常直连（F1） | ●●●●○（close 失败仅 Toast） | 通过 |
| 2 首连+保存（F2） | 保存需重填 | 部分缺失 |
| 3 连接失败（F4） | 密钥/密钥变更场景错位 | 部分缺失 |
| 4 会话中断（F5） | error 无提示无恢复 | **不通过** |
| 5 凭据沉淀（F6+异常） | 换机路径死路 | **不通过**（DS-01+UF-01 叠加） |

核心结论（USER_FLOW_REVIEW.md §五原文转引）：正常路径（F1）已达产品化水准；但三类异常恢复路径（凭据轮换 UF-01、运行时错误 UF-02、慢连接 UF-03）系统性缺失或死路。重开发的流程规格应把「异常恢复」从 P4 的 7 条 B 类扩展到包含本册 4 条 B 类的完整清单。

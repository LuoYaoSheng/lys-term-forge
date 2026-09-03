# TermForge 错误处理盘点与错误码规范（ERROR CODE）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 现状盘点来源：源码 grep（2026-09-03，全部 .catch/catch 点逐条列出）+ `docs/01_reverse/REVERSE_ANALYSIS.md` §④ + `docs/02_product/PAGE_SPEC.md` §1/§2.3；V1 错误码规范建议来源：`docs/08_development/API_SPEC.md` §2（P7 契约）与 `docs/06_review/PRODUCT_REVIEW.md` B-13/C-6/C-7。规范部分为「建议规格」，注明出处，未实施。

---

## 一、现状错误处理机制盘点

### 1.1 总体模式

- **后端**：命令返回 `Result<T, String>`——错误为**自由文本字符串**，无结构化错误码（API_SPEC §头部约定，源码事实）。
- **前端连接级错误**：`friendlyError` 按原始错误文本子串匹配映射为 6 种友好文案（TerminalTab L105-113）+ 终端红字 `[error]` 呈现。
- **前端操作反馈**：保存/删除失败走 Toast；表单校验走字段级红字（touched 后显示）。
- **事件流错误**：后端经 `app_event terminal:status{status:"error"}` 推送（读/写/resize 错误三种触发源，client.rs L242-289）。

### 1.2 前端全部 catch 点清单（2026-09-03 grep 实测，8 处）

| # | 位置 | 场景 | 处理方式 | 用户可见性 |
|---|---|---|---|---|
| 1 | `src-ui/src/lib/api.ts` L17 | onAppEvent 事件系统不可用 | 静默返回 no-op unlisten（注释自认） | **完全不可见**（事件流整体失效即终端无输出） |
| 2 | `src-ui/src/App.svelte` L154 | onMount 加载 connection_list 失败 | 静默（侧栏显示空列表） | **不可见**（用户误以为无已存连接） |
| 3 | `src-ui/src/App.svelte` L182 | closeTab 中 session_close 失败 | Toast "Failed to close session"，但 Tab 仍移除 | 可见（但后端句柄泄漏 FL-04/C-6） |
| 4 | `src-ui/src/components/TerminalTab.svelte` L46 | session_resize 失败 | **`.catch(() => {})` 静默吞** | **不可见**（PTY 尺寸失同步无感知） |
| 5 | `src-ui/src/components/TerminalTab.svelte` L84 | session_send 失败（键盘输入） | **`.catch(() => {})` 静默吞** | **不可见**——会话死后的输入全部静默丢弃（ST-01/UF-02 核心） |
| 6 | `src-ui/src/components/TerminalTab.svelte` L162 | connect/session_open 失败（含 15s 超时） | 状态置 error + 终端红字 friendlyError + Reconnect 条 | 可见 |
| 7 | `src-ui/src/components/ConnectionForm.svelte` L86 | connection_save 失败 | Toast "Failed to save connection" | 可见 |
| 8 | `src-ui/src/components/ConnectionForm.svelte` L100 | connection_delete 失败 | Toast "Failed to delete connection" | 可见 |

### 1.3 后端静默/降级点清单

| # | 位置 | 场景 | 处理方式 | 用户可见性 | 编号 |
|---|---|---|---|---|---|
| 9 | `src-tauri/src/commands/store.rs` L88-91 | list 时密码解密失败（换机/改 hostname/username） | `warn!` 后置 password=None | **不可见**（表现为"该连接无密码"） | DS-01（B） |
| 10 | `src-tauri/src/commands/store.rs` L104-107 | save 时加密失败 | `warn!` 后**明文落盘**（注释 "storing without encryption"） | **不可见**（触发条件近死，DS-02 校准） | C-7（V1 规格：拒绝保存） |
| 11 | `src-tauri/src/core/session_manager.rs` | IO 线程退出（EOF/读错误）后句柄残留、status 陈旧 | 无回调清理，仅 close() 一条移除路径 | 不可见（session_list 数据失真，前端又未调用） | ST-04（B） |
| 12 | `src-tauri/src/commands/store.rs` L48-52 | connections.json 文件损坏 | 兜底返回空列表（恒 Ok） | 不可见（数据"消失"无提示） | DATA_STORAGE_REVIEW §一 D1 |
| 13 | `src-tauri/src/main.rs` | tracing 日志仅 stdout | 解密失败/MITM 告警/密钥探测失败均只进日志流 | 打包后用户不可见 | DS-09（B） |

### 1.4 前端 friendlyError 映射表（现状 6 条，TerminalTab L105-113）

| 原始错误包含 | 显示 |
|---|---|
| Connection refused | Connection refused — check host and port |
| Authentication | Authentication failed — check username and password |
| timed out / timeout | Connection timed out |
| Name or service not known | Host not found — check the address |
| Network is unreachable | Network unreachable |
| 其他（兜底） | Connection failed |

已知缺口：不含 "Host key mismatch" 分支——MITM 警告被兜底吞掉（FL-10/B-13）；密钥认证失败被 "Authentication" 命中而提示错位（UF-05）。错误提示文案含未注册的 Ctrl+R 死文案（FL-01/B-04）。

## 二、错误码规范建议（V1 规格，未实施）

> 来源：`docs/08_development/API_SPEC.md` 头部约定与 §2.1——现状后端返回 `Result<T, String>`（自由文本）；V1 契约引入结构化错误 `{ code: string, message: string }`（是否 MVP 落地【建议，待用户确认】）。以下 code 均为 V1 规格值，与现状自由文本的归类对应关系见 API_SPEC §2.1。

### 2.1 错误码总表

| code | 触发（现状自由文本归类） | 前端映射（friendlyError） | 备注 |
|---|---|---|---|
| E_CONN_REFUSED | TCP 拒绝 | Connection refused — check host and port | |
| E_AUTH_FAILED | userauth 失败 / 无密码无可用密钥 | Authentication failed — …（+V1 B-08 指引） | UF-05：建议区分密码/密钥两类文案 |
| E_TIMEOUT | 前端 15s 竞速 / TCP 超时 | Connection timed out | ST-03：须配套取消/清理语义 |
| E_DNS | Name or service not known | Host not found — check the address | |
| E_NET_UNREACHABLE | Network is unreachable | Network unreachable | |
| E_HOSTKEY_MISMATCH | 指纹不一致 | **V1 专案（B-13 第七条）**：主机密钥变更——MITM 警告 + known_hosts 处置指引 | 依赖 PL-02 指纹口径勘误 |
| E_CONN_FAILED | 其他（含 Task join error） | Connection failed | 兜底 |
| E_SESSION_NOT_FOUND | "session not found"（session_send/resize） | —（现状静默） | 幂等语义 |
| E_SESSION_CLOSED | mpsc send 失败 "SSH session closed" | —（现状 `.catch` 静默） | ST-01：V1 须映射状态而非吞掉 |
| E_ENCRYPT_FAILED | 加密失败 | V1：Toast「密码加密失败，连接未保存」，**不写盘** | C-7 修复后规格（替代明文降级） |
| E_IO | 写盘/权限失败（connection_save/delete、host_key_trust） | Toast Failed to save/delete connection | |

### 2.2 配套规范要点（各出处转引）

1. **新增事件/错误必须双端登记**：models/events.rs 枚举 + api.ts AppEvent 联合类型，禁单侧；禁止前端预留无后端实现的类型（API_SPEC §3，旧仓 3 个残留事件为红线）。
2. **错误映射单源化**：friendlyError 从 TerminalTab 内联 6 分支抽为 `lib/errors.ts` 单源（7 分支含 B-13），供终端/Toast 复用（MODULE_ARCH.md §1/§2 公共能力归位表）。
3. **静默吞错点治理**（对照 §1.2 清单）：
   - #5 session_send 静默：至少一次 Toast + 状态映射（ST-01/UF-02/PL-05——读错误必须断态，写错误可仅提示不断态）；
   - #4 session_resize 静默：保留静默可接受（低危），但应在 V1 明确豁免理由并注释；
   - #2 connection_list 加载失败：V1 至少 Empty 列表 + 一次性提示（connectionStore load 失败策略，STATE_MACHINE.md §3.2）;
   - #1 onAppEvent no-op：事件系统不可用属致命环境错误，应显式呈现而非 no-op。
4. **会话生命周期错误语义**：session_close 幂等（不存在 id 记 warn 返回 Ok）+ 先移除后关闭保证回收（API_SPEC §2.3 V1 契约，修复 FL-04/C-6 泄漏）。
5. **脱敏铁律**：Debug 输出密码 `***`（dto.rs 手工 Debug）保留；禁止明文密码进日志/Debug（GUIDELINES.md §5）。
6. **密钥绑定失配错误应显式化**：decrypt 失败区分 `password: none / undecryptable` 两态，UI 显示「密码无法在本机解密（可能已迁移设备），请重新输入」（DS-01 建议方向）。

## 三、盘点结论（引评审汇总，不新增）

PRODUCT_LOGIC_REVIEW.md §二验收项：8 项中与错误处理直接相关的「流程完整（异常三类齐备）」不达标（10 项中 6 项缺下一步或恢复）。UX 层修复优先级见 `docs/06_review/UX_REVIEW.md` §四：运行时错误处理链路（PL-05）列第一。

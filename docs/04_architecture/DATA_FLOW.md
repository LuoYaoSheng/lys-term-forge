# TermForge 数据流动（DATA FLOW）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 内容来源：实体与流向取自 `docs/08_development/DATA_MODEL.md`（P7 产物，字段级来源已注明）、`docs/01_reverse/REVERSE_ANALYSIS.md` §②⑦、`docs/04_architecture/SYSTEM_ARCH.md` §1；风险点引 `docs/product-review/DATA_STORAGE_REVIEW.md`（DS 编号）。密钥绑定 hostname+username 为源码事实（`src-tauri/src/core/crypto.rs` L10-20）。

---

## 1. 端到端数据流总图

```mermaid
flowchart TB
    subgraph FE["前端 src-ui（Svelte 4）"]
        FORM["ConnectionForm 表单<br/>host/port/username/password"]
        APP["App.svelte<br/>tabs[] / savedConnections / activeView"]
        TERM["TerminalTab<br/>xterm 实例（scrollback 5000）"]
        API["lib/api.ts<br/>invoke 封装 + onAppEvent"]
        TOAST["lib/toast.ts"]
    end

    subgraph BR["Tauri 2 桥"]
        CMD["invoke 命令（8 个）"]
        EVT["事件 app_event<br/>（单一事件名 + type 标签）"]
    end

    subgraph BE["后端 src-tauri（Rust）"]
        STORE["commands/store.rs<br/>ConnectionStoreManager"]
        SM["core/session_manager.rs<br/>HashMap&lt;session_id, SessionHandle&gt;"]
        SSH["core/ssh/client.rs<br/>SSHSession + IO 线程"]
        CRYPT["core/crypto.rs<br/>AES-256-GCM"]
        FS_HOME["~/.termforge/connections.json<br/>~/.termforge/known_hosts（均 0600）"]
    end

    REMOTE["远端 SSH 服务器（用户主动连接）"]

    FORM -->|Connect: 连接参数快照| APP
    APP -->|createTab| TERM
    FORM -->|Save / Delete| API
    APP -->|connection_list 镜像| API
    TERM -->|session_open/send/resize/close| API
    API <--> CMD
    CMD --> STORE
    CMD --> SM
    SM --> SSH
    STORE <--> CRYPT
    STORE -->|pretty JSON 读写| FS_HOME
    SSH -->|TOFU 指纹读写| FS_HOME
    SSH <-->|"TCP + ssh2（握手/认证/PTY/IO）"| REMOTE
    SSH -->|emit_status / terminal:data| EVT
    EVT -->|listen + session_id 过滤| TERM
    TERM -->|write(chunk)| TERM
    API -->|操作结果| TOAST
```

（流向来源：SYSTEM_ARCH.md §1 总体架构图 + REVERSE_ANALYSIS §② 事件流架构；TEXT 注：`HashMap<...>` 中 `<` 转义为 `&lt;` 以兼容 Mermaid。）

## 2. 三条核心数据链路

### 2.1 连接配置 → 加密存储 → 读取回显

| 步骤 | 动作 | 实现位置（来源：DATA_MODEL.md §2.1、API_SPEC §2.6-2.8） |
|---|---|---|
| 写入 | 前端校验+查重（name 或 host+port+username）→ `connection_save(SavedConnection{id: conn_{uuid}, name: username@host, host, port, username, password?})` | ConnectionForm → commands/store.rs save()：按 id upsert → pretty JSON 写 `~/.termforge/connections.json`（Unix 0600） |
| 加密 | password 经 `encrypt()`：`stored = base64(nonce[12] OsRng 随机 ‖ ciphertext ‖ tag[16])`；**key = SHA-256("TermForge-v1:{hostname}:{username}")**（32 字节，机器绑定） | core/crypto.rs |
| 读取 | `connection_list` → 逐条 `decrypt()` 还原明文密码返回前端（App.svelte savedConnections 镜像；选择连接回填表单） | store.rs list() L80-96 |
| 删除 | `connection_delete(id)` → retain 过滤后重写文件（幂等） | store.rs |

**链路已知风险**（来源：DATA_STORAGE_REVIEW.md）：
- **密钥绑定 hostname+username 的事实**：换机器、改用户名、改主机名、克隆用户配置目录，任一发生即**全部已存密码解密失败**（decrypt 错误信息自述 "data may be corrupted or from another machine"，crypto.rs L61）；list 时解密失败静默置 password=None（DS-01，B 类，高频真实数据丢失）。
- 加密失败存在明文降级分支（store.rs L104-107）——但触发条件近死（密钥恒 32 字节、AES-GCM 仅超长明文失败）且降级数据下次读取必被解密链路吞掉（DS-02，PL-04 校准）。
- connection_list 返回**解密后明文密码**，在前端 4 处常驻（savedConnections、每 Tab connection 快照、表单回填、双击 dispatch 载荷）——DS-03（C 类）。
- 存储无 schema 版本与密文标记（明文/密文不可区分）——DS-04；V1 建议 `{schema_version, connections[]}` + `enc:v1:` 前缀。

### 2.2 连接参数 → SSH 会话建立 → 输入输出流

| 步骤 | 动作 | 实现位置（来源：REVERSE_ANALYSIS §② 核心模块表、API_SPEC §2.1-2.5） |
|---|---|---|
| 1 | 前端创建 Tab（connection 快照）→ TerminalTab connect() → `session_open{host,port,username,password?,key_path?}`（15s Promise.race） | App.svelte → TerminalTab |
| 2 | 后端 `spawn_blocking` 包裹：TCP connect（std::net::TcpStream）→ ssh2 handshake → 主机密钥验证（TOFU）→ 认证（密码/显式 key_path/默认 ~/.ssh 密钥探测 id_ed25519/id_rsa/id_ecdsa）→ channel_session + request_pty("xterm-256color", 80×24) + shell | core/ssh/client.rs SSHSession::new |
| 3 | 成功：生成 `session_id = ssh_{nanoid(10)}` 入 HashMap（status 写死 "connected"）→ 返回前端 → emit `terminal:status connected` | session_manager.rs L70-84 |
| 4 | 启动专用 OS 线程：5ms 轮询非阻塞读（8KB 缓冲，UTF-8 lossy）→ emit `terminal:data{session_id, chunk}`；mpsc 接收 Write/Resize/Close 命令 | client.rs IO 线程 L207+ |
| 5 | 前端按 session_id 过滤事件 → xterm write 渲染；键盘输入 → `session_send` → mpsc Write；窗口/字号变化 → fit → `session_resize` | TerminalTab |

**链路已知风险**：运行时 error 事件不进前端状态机（ST-01）；后端 SessionHandle.status 写后不更新、IO 线程退出无回调清理（ST-04）；15s 前端超时非取消协议 → 慢连接状态分裂+孤儿会话（ST-03）。详见 `docs/03_flow/BUSINESS_FLOW.md` §3。

### 2.3 主机密钥指纹（TOFU）流

| 步骤 | 动作 | 实现位置（来源：DATA_MODEL.md §2.4、TOKEN.md §8.4） |
|---|---|---|
| 读取 | 每次连接：TCP+握手后、用户认证前，读 `~/.termforge/known_hosts`（行格式 `host:port 指纹`，0600）比对 | client.rs verify_host_key L39-84 |
| 首连 | 无记录 → 自动追加指纹并信任（用户无感知） | client.rs L65-74（PM-03：安全决策静默代理） |
| 匹配 | 完全相等 → 通过（info 日志） | 同上 |
| 变更 | 不等 → 连接失败，错误含 "Host key mismatch … possible man-in-the-middle attack"（前端被兜底吞，FL-10/B-13） | 同上 |

**口径勘误**（PL-02，A 类）：指纹实为**服务器公钥原始字节的逐字节 hex**（冒号分隔），**非 SHA-256 哈希指纹**（client.rs L22-33 注释与四份文档均写 SHA-256，与实现不符）——重开发若照文档实现将与既有 known_hosts 数据静默不兼容。

## 3. 数据所有权与生命周期总表

来源：DATA_MODEL.md §4（原文转引）。

| 数据 | 所有者 | 生命周期 |
|---|---|---|
| SavedConnection | 后端 ConnectionStoreManager（内存 + connections.json） | 应用级持久 |
| known_hosts | 后端 ssh/client 直接读写文件 | 应用级持久 |
| SessionHandle | 后端 SessionManager | 会话级（close/断线即除名——**现状：仅 close() 一条移除路径，断线残留见 ST-04**） |
| Tab（含 connection 快照/明文密码） | 前端 App 状态 | 窗口级（关 Tab 即除名，不持久化——FR48 状态恢复为规划） |
| Toast | 前端 toast store | 3s/250ms |
| 终端缓冲 | xterm 实例（scrollback 5000） | Tab 级（dispose 即清） |

## 4. 事件与命令契约速查

- 命令（8 个，`src-tauri/src/lib.rs` L16-27）：session_open / session_send / session_close / session_list / session_resize / connection_list / connection_save / connection_delete。
- 事件（单一 `app_event`，type 标签）：`terminal:data{session_id, chunk}`、`terminal:status{session_id, status, msg?}`——后端仅此两种已实现；前端 api.ts 另预留 sftp:progress / runbook:progress / monitor:snapshot（规划残留，新仓红线：禁止无后端实现的类型，API_SPEC §3）。
- 规则：新增事件必须同时登记 `models/events.rs` 枚举与 `api.ts` AppEvent 联合类型，禁单侧（MODULE_ARCH.md §5）。

## 5. 无云、无遥测的数据边界

应用运行时仅发起用户主动的 SSH TCP 连接，无遥测/无更新检查/无云服务（REVERSE_ANALYSIS §⑧）；所有用户数据（连接库/known_hosts/偏好）均落在本机 `~/.termforge/`（偏好现状仅内存不落盘，DS-08/F034）。用户数据零出口（无导出/备份/迁移，DS-06）。

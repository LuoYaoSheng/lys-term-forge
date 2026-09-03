# TermForge 时序图集（SEQUENCE DIAGRAMS）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 内容来源：交互步骤取自 `docs/08_development/API_SPEC.md` §2/§4（P7 契约）、`docs/01_reverse/REVERSE_ANALYSIS.md` §②④、`docs/04_architecture/STATE_MACHINE.md` §2/§4；缺陷行为（超时分裂/静默吞错等）如实标注并给出处编号（ST/DS/UF，见 docs/product-review/ 各分册）。现状行为与 V1 修复后规格在图中以 Note 区分。

---

## 时序 1：新建连接 + 认证（session_open 全链路）

现状（自动 TOFU）；V1 建议为确认式 TOFU（host_key_check/host_key_trust 两命令为 V1 新增规格，随 B-12 决策，尚未实现）。

```mermaid
sequenceDiagram
    participant U as 用户
    participant FE as 前端 ConnectionForm/TerminalTab
    participant BE as 后端 commands/session.rs
    participant CORE as core/ssh/client.rs
    participant SSH as 远端 SSH 服务器

    U->>FE: 填写 host/port/username/password 点击 Connect
    FE->>FE: 校验 Host必填/Port 1-65535/Username必填
    alt 校验失败
        FE-->>U: 字段下方红字提示（touched 后显示）
    else 校验通过
        FE->>FE: App 创建 Tab 状态 idle → connecting（15s Promise.race 计时开始）
        FE->>BE: session_open(SessionOpenRequest)
        BE->>CORE: spawn_blocking 包裹连接
        CORE->>SSH: TCP connect（std::net::TcpStream）
        CORE->>SSH: ssh2 handshake
        CORE->>CORE: 主机密钥验证 TOFU（~/.termforge/known_hosts）
        alt 首次连接（无记录）
            CORE->>CORE: 自动追加指纹并信任（用户无感知 PM-03）
        else 指纹匹配
            CORE->>CORE: 通过（info 日志）
        else 指纹变更
            CORE-->>BE: 错误含 MITM 警告
            BE-->>FE: Err（前端兜底显示 Connection failed FL-10/B-13）
        end
        CORE->>SSH: 认证（密码 / 显式 key_path / 默认 ~/.ssh 密钥探测）
        alt 认证成功
            CORE->>SSH: request_pty xterm-256color 80×24 + shell
            CORE->>CORE: 启动专用 IO 线程（5ms 轮询读 + mpsc 命令）
            CORE-->>BE: SSHSession 就绪
            BE->>BE: 生成 session_id ssh_{nanoid10} 入 HashMap
            BE-->>FE: Ok(SessionOpenResponse session_id)
            BE-->>FE: app_event terminal:status connected
            FE->>FE: 状态 connected 清屏 + 回填 sessionId + 初始 doFit/resize
        else 认证失败/拒绝/DNS/超时
            BE-->>FE: Err（自由文本）
            FE->>FE: friendlyError 6 种映射 → 状态 error + 红字 + Reconnect 条
        end
    end
```

（步骤来源：API_SPEC §2.1、REVERSE_ANALYSIS §④ PAGE003、client.rs 认证三分支 L135-186。）

## 时序 2：会话建立后的输入输出循环

```mermaid
sequenceDiagram
    participant U as 用户
    participant FE as TerminalTab(xterm)
    participant BE as 后端
    participant IO as IO 线程（每会话 1 条）
    participant SSH as 远端

    U->>FE: 键盘输入
    FE->>BE: session_send(session_id, data)
    BE->>IO: mpsc IoCommand::Write
    alt 会话已死（mpsc send 失败）
    BE-->>FE: Err（前端 .catch 静默吞 ST-01）
    else 正常
    IO->>SSH: channel.write
    SSH-->>IO: 输出数据
    loop 5ms 轮询非阻塞读（8KB 缓冲 UTF-8 lossy）
        IO-->>FE: app_event terminal:data(session_id, chunk)
        FE->>FE: 按 session_id 过滤 → xterm.write 实时渲染
    end
    U->>FE: 拖拽窗口/调整字号
    FE->>FE: FitAddon.fit 计算 cols×rows
    FE->>BE: session_resize(session_id, cols, rows)
    BE->>IO: mpsc IoCommand::Resize
    IO->>SSH: request_pty_size
    alt resize 失败
        IO-->>FE: app_event terminal:status error "Resize error ..."
    end
    end
```

（步骤来源：REVERSE_ANALYSIS §② IO 模型、API_SPEC §2.2/§2.5；send 失败静默 catch 为 TerminalTab L84 事实，ST-01。）

## 时序 3：断线与错误路径

```mermaid
sequenceDiagram
    participant SSH as 远端
    participant IO as IO 线程
    participant FE as TerminalTab
    participant APP as App.svelte

    alt 远端正常关闭（EOF）
        SSH-->>IO: EOF
        IO-->>FE: app_event terminal:status closed "Connection closed by remote"
        FE->>FE: writeln [status] closed 行 → 状态 closed（灰点）
        Note over FE: closed 态无 Reconnect 按钮（能力在入口漏 FL-02/B-05）
    else 网络读错误
        SSH-->>IO: read error
        IO-->>FE: app_event terminal:status error "Read error ..."
        FE->>FE: 仅打印一行 [status] error 文本 状态不变（仍 connected 绿点）
        Note over FE: IO 线程已退出 后续输入静默丢弃（ST-01/UF-02 P4 未覆盖）
        Note over IO: 后端 HashMap 句柄残留 status 陈旧 connected（ST-04）
    else 写错误
        SSH-->>IO: write error
        IO-->>FE: app_event terminal:status error "Write error ..."
        Note over FE: 报 error 事件但连接保持 不断连（现状行为保留）
    else 前端 15s 超时且后端 15-30s 间连上（慢连接）
        FE->>FE: Promise.race 超时 → 状态 error 报 Connection timed out
        IO-->>FE: terminal:status connected（session_id 不匹配被过滤）
        Note over FE: UI=error 与后端=connected 长期并存 孤儿会话（ST-03/UF-03）
    end
    APP->>APP: 非激活 Tab 断线仅状态点变灰 无全局信号（FL-11）
```

（步骤来源：REVERSE_ANALYSIS §⑥ 流程 3、STATE_MACHINE.md §1.1/§1.3 双轨对照表、client.rs L216-289 事件触发点。）

## 时序 4：手动重连（Reconnect）

```mermaid
sequenceDiagram
    participant U as 用户
    participant FE as TerminalTab
    participant BE as 后端
    participant SSH as 远端

    Note over FE: 前置 状态为 error 或 closed
    U->>FE: 点击 Reconnect 按钮（V1 修复后规格 closed 态亦有按钮 B-05）
    FE->>FE: 若有旧 unlisten 先取消（竞态守卫 TerminalTab L117）
    FE->>FE: 状态 → connecting 终端显示 Connecting...
    FE->>BE: session_open(凭据=Tab 创建时的 connection 快照)
    Note over FE,BE: 快照凭据 表单已改不影响重连（UF-01 三重锁死之一）
    alt 成功
        BE-->>FE: 新 session_id（重连换新 id）
        BE-->>FE: terminal:status connected
        FE->>FE: 状态 connected 派发 connected 事件回写 App.tabs.sessionId
    else 失败
        BE-->>FE: Err → friendlyError → 状态 error
        Note over FE: 认证失败场景须关 Tab 重建（改凭据无效 B-08 指引）
    end
    opt 旧会话仍占后端句柄（慢连接场景重连）
        Note over BE: 旧会话无人 close → 孤儿会话累积（ST-03）
    end
```

（步骤来源：STATE_MACHINE.md §2.2 单一写入者/重连换 id/竞态守卫、USER_FLOW_REVIEW.md F3/UF-01。）

## 时序 5：凭据存取加解密（Save / list / 换机失效）

```mermaid
sequenceDiagram
    participant U as 用户
    participant FE as ConnectionForm
    participant ST as commands/store.rs
    participant CR as core/crypto.rs
    participant FS as ~/.termforge/connections.json

    rect rgb(30,30,46)
    Note over U,FS: 保存路径（Save）
    U->>FE: 填表点 Save
    FE->>FE: 校验 + 查重（name 或 host+port+username）
    alt 查重命中
        FE-->>U: Toast already exists（不写入）
    else 通过
        FE->>ST: connection_save(SavedConnection)
        ST->>CR: encrypt(password)
        CR->>CR: key = SHA-256("TermForge-v1:{hostname}:{username}")
        CR->>CR: base64(nonce12 ‖ ciphertext ‖ tag16)
        alt 加密失败（触发条件近死 DS-02）
            ST->>FS: 明文落盘（warn 降级 缺陷 V1 改为拒绝保存 E_ENCRYPT_FAILED）
        else 成功
            ST->>FS: pretty JSON 写入（0600）按 id upsert
            ST-->>FE: Ok
            FE-->>U: Toast saved + 刷新列表
        end
    end
    end

    rect rgb(30,30,46)
    Note over U,FS: 读取路径（应用启动 / 保存删除后刷新）
    FE->>ST: connection_list
    ST->>FS: 读取 JSON（损坏则空列表兜底）
    loop 每条记录
        ST->>CR: decrypt(password)
        alt 解密成功（本机 hostname+username 未变）
            CR-->>ST: 明文密码
        else 解密失败（换机/改用户名/改主机名）
            ST->>ST: warn 后置 password=None（静默 DS-01）
            Note over ST: 前端表现为该连接无密码 无任何提示
        end
    end
    ST-->>FE: Vec<SavedConnection>（password 为解密明文 DS-03）
    FE->>FE: App.savedConnections 镜像 选择连接回填表单
    end
```

（步骤来源：API_SPEC §2.6-2.7、DATA_MODEL.md §2.1/§2.6、DATA_STORAGE_REVIEW.md §2.1-2.3；加密参数出处 crypto.rs。）

## 时序 6：关闭会话（session_close）

```mermaid
sequenceDiagram
    participant U as 用户
    participant APP as App.svelte
    participant BE as 后端
    participant IO as IO 线程

    U->>APP: Ctrl+W / 点击 Tab ×
    alt Tab 无 sessionId
        APP->>APP: 直接移除 Tab
    else Tab 有 sessionId
        APP->>BE: session_close(session_id)
        alt 成功
            BE->>IO: mpsc IoCommand::Close
            IO->>IO: channel.close → emit closed 事件 → 线程退出
            BE-->>APP: Ok
        else 失败（现状缺陷 FL-04/C-6）
            BE-->>APP: Err
            APP->>APP: Toast Failed to close session 但 Tab 仍移除
            Note over BE: SessionManager 句柄永不清理（泄漏）<br/>V1 修复后规格 先移除后关闭 幂等 保证回收
        end
    end
    APP->>APP: 激活相邻 Tab（prevIndex 钳制）或显示 PAGE009 空态
```

（步骤来源：API_SPEC §2.3 现状缺陷与 V1 契约、REVERSE_ANALYSIS §⑥ 流程 3、App.svelte L177-194。）

## 时序总览与状态机对应

| 时序 | 对应状态迁移（STATE_MACHINE.md §2.1） | 备注 |
|---|---|---|
| 1 新建连接+认证 | idle → connecting → connected / error | 15s 前端竞速 + TCP 30s 后端超时双层 |
| 2 输入输出循环 | connected 保持 | terminal:data 无状态变化 |
| 3 断线/错误 | connected → closed（仅此一条运行时事件路径）；error 事件**不迁移**（ST-01 缺口） | api.ts 类型联合不含 error |
| 4 重连 | closed/error → connecting → connected/error | 换新 session_id |
| 5 凭据加解密 | 不涉及 Tab 状态 | 后端文件读写 |
| 6 关闭会话 | closed/error → [*]（closeTab） | 幂等修复为 V1 规格 |

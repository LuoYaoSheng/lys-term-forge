# TermForge 权限评审（PERMISSION REVIEW）

> 评审依据：《AI 产品重构逻辑评审规范 v1.0》· 2026-09-03
> 输入文档清单：docs/01_reverse/REVERSE_ANALYSIS.md（P1）、docs/02_product/PRD.md（P2）、docs/02_product/PAGE_SPEC.md（P3）、docs/06_review/PRODUCT_REVIEW.md（P4）；源码抽查 src-tauri/capabilities/default.json、src-tauri/tauri.conf.json、src-tauri/src/lib.rs、src-tauri/src/core/ssh/client.rs、src-tauri/src/commands/store.rs、src-ui/src/lib/api.ts 及 src-ui/src 全目录 grep（shell 调用点核查）。
> 铁律遵守：只评审不修改；所有「当前设计」均附源码依据；无法证实处标【未知】。

---

## 一、系统权限清单（OS / 运行时层）

| # | 权限项 | 申请/使用方式 | 依据 | 必要性评审 |
|---|---|---|---|---|
| S1 | 网络出站（TCP，用户指定 host:port） | Rust 侧 TcpStream::connect，不经 OS 授权弹窗（桌面应用常规） | client.rs L123 | 必要——产品本质（SSH 客户端）。注意范围无任何约束（见 PM-04）。 |
| S2 | 文件系统写 `~/.termforge/`（目录+两文件） | 静默 create_dir_all + 写文件（无系统对话框） | store.rs L72-78；client.rs L13-20 | 必要且克制（应用自有目录，未散落配置）。 |
| S3 | 文件系统**读 `~/.ssh/` 私钥探测** | 无密码无 key_path 时静默尝试 id_ed25519/id_rsa/id_ecdsa | client.rs L152-179 | **隐式使用高敏感凭据材料**（PM-02）。 |
| S4 | WebView 能力：core:default、event listen/emit、window start-dragging | capabilities 声明 | capabilities/default.json L7-10 | 必要（基础窗口/事件）。 |
| S5 | shell:allow-open（打开外部 URL/文件） | capabilities 声明 + tauri_plugin_shell 注册 | capabilities/default.json L11；lib.rs L13 | **授权未使用**（PM-01）：grep src-ui/src 全目录无 plugin-shell 导入或 open 调用（api.ts 仅 invoke/listen/getCurrentWindow）。 |
| S6 | CSP 内容安全策略 | default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline' | tauri.conf.json L24-25 | 已收紧，正面。style 'unsafe-inline' 为内联样式所需，常见取舍。 |
| S7 | 未申请：剪贴板、系统通知、开机自启、文件对话框、全局快捷键、摄像头/麦克风 | 无对应 capability/插件 | capabilities/default.json 全文 | 克制，正面；剪贴板能力缺失影响 F044（功能层）。 |

**系统权限结论**：申请面总体克制（S7 正面）；两处问题：S5 过度授权（授权了未使用的 shell open）、S3 隐式敏感读取（未告知）。

---

## 二、应用内权限清单（产品层）

| # | 权限点 | 当前设计 | 依据 | 评审 |
|---|---|---|---|---|
| A1 | 账户/角色/多用户 | 无账户体系，单用户本地应用 | 全代码无认证/授权概念 | 合理（定位所致）。 |
| A2 | 破坏性操作确认 | 仅「删除连接」有原生 confirm() | ConnectionForm L94 | 覆盖面=当前全部破坏性操作（删除连接是唯一项），合格；confirm 形态问题见 P4 F-05/B-06。 |
| A3 | 主机密钥信任决策（TOFU） | 首连**自动信任并落盘**，无确认 | client.rs L65-84 | **安全决策被应用静默代理**（PM-03；P4 FL-09/B-12 交叉引用）。 |
| A4 | SSH 私钥使用决策 | 无密码时自动探测并使用 | client.rs L152-179 | 同 A3，静默代理（PM-02）。 |
| A5 | 危险命令确认（rm -rf 等） | 无任何实现 | 无代码（F042 规划） | 终端输入直通远端；属功能取舍（PL-01），非既有权限滥用。 |
| A6 | 凭据可见性 | password 输入框掩码；列表/Tab 不展示密码明文；但 connection_list 回传解密明文（内存） | ConnectionForm L198；store.rs L80-96 | 呈现层合格；内存暴露面归 DS-03。 |

**应用内权限结论**：无角色体系（合理）；「需要用户授权/确认的决策点」共识别 4 个（删除连接、信任主机密钥、使用私钥、执行危险命令），当前仅覆盖第 1 个。

---

## 三、问题清单

> 格式：当前设计 / 问题 / 影响 / 建议方向。分级沿用 A/B/C/D。

### PM-01【B】shell:allow-open 过度授权（授权与使用不符）
- **当前设计**：capabilities/default.json L11 声明 `shell:allow-open`，lib.rs L13 注册 tauri-plugin-shell；前端全目录 grep 无任何 plugin-shell 导入或 open 调用（api.ts 仅使用 @tauri-apps/api 的 invoke/listen/getCurrentWindow）。
- **问题**：最小权限原则下，未被使用的shell 打开能力构成多余攻击面（一旦前端被注入脚本，可借此拉起外部 URL/协议处理器；虽然 CSP 收敛了脚本来源，纵深防御仍不应保留）。
- **影响**：低概率高影响的攻击面冗余；同时向重开发者传递「产品需要 shell open」的错误信号。
- **建议方向**：移除该授权（若规划中的「打开日志目录/外部链接」需要，届时随功能恢复并限定 domain/命令白名单）。

### PM-02【C】~/.ssh 私钥隐式探测使用，无任何告知或授权点
- **当前设计**：连接请求不含密码时，后端自动读取并尝试 ~/.ssh/id_ed25519、id_rsa、id_ecdsa 完成认证（client.rs L152-179）；UI 侧无任何提示——密码框连 placeholder 都没有（ConnectionForm L196-199）。
- **问题**：私钥是用户最高敏感度的凭据材料；「应用代替用户决定使用其私钥」在权限逻辑上是一个应显式化的决策点。P4 B-09 已在原型层补密码字段辅助文案（呈现层），但「是否/如何授权密钥使用」未被决策。
- **影响**：安全敏感用户（恰好是 SSH 工具的核心用户）的信任成本；审计场景（企业）不可解释「何时用了哪把钥匙」。
- **建议方向**：随 F045 密钥认证 UI（C 类）一并决策：显式 key_path 选择为主、默认探测为可选偏好并首次使用时提示；后端日志已有 per-key 记录（client.rs L168）可作审计基础。

### PM-03【B】TOFU 首连自动信任：安全决策静默代理（P4 FL-09/B-12 交叉引用）
- **当前设计**：见 A3。
- **补充视角**：从权限评审看，这是「应用代替用户行使信任授权」；ssh 客户端行业惯例（OpenSSH 的 yes/no 确认）证明该决策点历史上就归用户。
- **建议方向**：与 P4 B-12 一致（确认式 TOFU）；重开发规格中将「主机密钥确认」列为连接流程的必备环节，并联动 UF-04 的指纹口径勘误。

### PM-04【D】网络出站目标范围无约束
- **当前设计**：用户可连任意 host:port（含内网/公网/本地）；无目标白名单/告警概念。
- **问题/影响**：单用户运维工具属产品本质，加约束反而伤害可用性；仅在企业审计场景构成话题。
- **建议方向**：观察不做；若未来有团队版诉求再议。

### PM-05【D】无剪贴板/通知等系统权限申请（正面基线记录）
- **当前设计**：见 S7。
- **说明**：记录为正面基线——权限申请面与当前功能严格匹配（除 PM-01 一处冗余）；复制粘贴缺失是功能缺口（F044），不是权限滥用。重开发引入剪贴板能力时按最小范围申请。

---

## 四、小结

| 分级 | 数量 | 编号 |
|---|---|---|
| B | 2 | PM-01、PM-03 |
| C | 1 | PM-02 |
| D | 2 | PM-04、PM-05 |

权限层最关键结论：**系统权限申请面克制（值得保持）；真正的权限问题在「决策点代理」——TOFU 信任（PM-03）与私钥隐式使用（PM-02）两个本应属于用户的安全决策被代码静默行使，加上一处冗余授权（PM-01）。重开发时应把「用户授权点清单」纳入连接流程规格：信任主机、使用密钥、（未来的）危险命令确认**。

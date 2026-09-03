# TermForge 重构文档索引（DOCUMENT INDEX）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》编号文档体系整理）
> 体系：00_context～09_test 十个编号目录 + prototype/{v0-old,v1-new} + product-review/ 六件套（目录位置不动）。
> 说明：`docs/` 下另有项目自有 VitePress 文档站（index.md / package.json / .vitepress/ / public/ / node_modules/）与 `docs/review/termforge-v1-final.png`（V1 验收截图，遗留位置），均非本套编号体系产物、未做改动。

---

## 一、目录树

```text
docs/
├── DOCUMENT_INDEX.md                      # 本索引
├── 00_context/                            # 项目上下文（P8 补齐）
│   ├── PROJECT_CONTEXT.md
│   ├── TECH_STACK.md
│   ├── ASSET_INVENTORY.md
│   └── DEPENDENCY_LIST.md
├── 01_reverse/
│   └── REVERSE_ANALYSIS.md                # 逆向分析报告（事实基线根文档）
├── 02_product/
│   ├── PRD.md
│   ├── PAGE_SPEC.md
│   ├── PRODUCT_MODEL.md                   # P8 补齐
│   └── FEATURE_MAP.md                     # P8 补齐
├── 03_flow/                               # P8 补齐（整目录）
│   ├── USER_FLOW.md
│   ├── PAGE_FLOW.md
│   └── BUSINESS_FLOW.md
├── 04_architecture/
│   ├── SYSTEM_ARCH.md
│   ├── MODULE_ARCH.md
│   ├── STATE_MACHINE.md
│   └── DATA_FLOW.md                       # P8 补齐
├── 05_sequence/
│   └── SEQUENCE_DIAGRAMS.md               # P8 补齐
├── 06_review/
│   ├── PRODUCT_REVIEW.md                  # P4 产品体验审查
│   ├── UX_REVIEW.md                       # P8 补齐（综合 P4 + UF 分册）
│   └── IA_REVIEW.md                       # P8 补齐（综合 IA 分册）
├── 07_design_system/
│   ├── TOKEN.md
│   ├── COMPONENT.md
│   ├── PATTERN.md
│   ├── ASSETS.md
│   └── GUIDELINES.md
├── 08_development/
│   ├── DATA_MODEL.md
│   ├── API_SPEC.md
│   ├── ERROR_CODE.md                      # P8 补齐
│   └── PERMISSION.md                      # P8 补齐
├── 09_test/
│   ├── COVERAGE_CHECKLIST.md
│   ├── HTML_V0_ACCEPTANCE.md
│   └── V1_ACCEPTANCE.md
├── product-review/                        # 产品逻辑评审六件套（目录位置不动）
│   ├── PRODUCT_LOGIC_REVIEW.md
│   ├── INFORMATION_ARCHITECTURE_REVIEW.md
│   ├── USER_FLOW_REVIEW.md
│   ├── DATA_STORAGE_REVIEW.md
│   ├── STATE_REVIEW.md
│   └── PERMISSION_REVIEW.md
├── review/
│   └── termforge-v1-final.png             # V1 验收截图（遗留位置，项目产物）
└── （项目自有：index.md / package.json / .vitepress/ / public/ / node_modules/）

prototype/
├── v0-old/app-prototype.html              # V0 旧版事实基线原型（只读）
└── v1-new/app-prototype.html              # V1 新版原型（B 类优化落地）
```

## 二、逐文件一行说明

### 00_context 项目上下文

| 文件 | 说明 |
|---|---|
| PROJECT_CONTEXT.md | 项目定位 / 仓库布局（src-ui 与 src-tauri 实测）/ 构建运行方式 / 关键入口 / 旧文档死链警示 |
| TECH_STACK.md | 技术栈清单（Tauri 2 + Svelte 4 + Rust/ssh2 + Tokio + AES-GCM，经 package.json 与 Cargo.toml 核实）+ V1 选型结论 |
| ASSET_INVENTORY.md | 资产清单：编号文档 / 原型 / 截图 / 应用图标 / 文档站 / 过程资产 / 运行时数据 |
| DEPENDENCY_LIST.md | 前后端依赖 + 用途 + 死依赖标注（tauri-plugin-shell、svelte-preprocess、tokio net feature） |

### 01_reverse 逆向

| 文件 | 说明 |
|---|---|
| REVERSE_ANALYSIS.md | 逆向分析报告 v1.0：①项目概述 ②结构 ③页面清单 ④页面详析 ⑤功能清单 F001-F043 ⑥用户流程 ⑦数据模型 ⑧依赖 ⑨规划对照与死链/矛盾/bug 记录——**全体系事实基线根文档** |

### 02_product 产品

| 文件 | 说明 |
|---|---|
| PRD.md | 产品需求文档：46 功能（F001-F046）+ 10 页面需求 + Given-When-Then 验收标准 |
| PAGE_SPEC.md | 页面交互规格：全局约定（令牌/快捷键/状态点/事件契约）+ 每页 11 维度 + 六项特检矩阵 |
| PRODUCT_MODEL.md | 产品定位 / 用户画像 P1-P4 / 使用场景 S1-S7 / 核心价值（P8 补齐） |
| FEATURE_MAP.md | 产品能力树：F001-F046 → 应用外壳/连接管理/SSH 终端/凭据存储/规划功能域/会话信息 分组（P8 补齐） |

### 03_flow 流程（P8 补齐整目录）

| 文件 | 说明 |
|---|---|
| USER_FLOW.md | 用户旅程 5 组（Mermaid）：日常直连 / 首连保存 / 失败处置 / 中断处置 / 凭据沉淀 |
| PAGE_FLOW.md | 页面/视图跳转关系：出入口表 + 跳转图 + 快捷键触达 + 可预测性评审要点 |
| BUSINESS_FLOW.md | 正常/异常/边界业务流：已知缺陷标注（运行时 error 不进状态机、解密失败静默吞密码、超时孤儿会话等 7+ 项带编号登记） |

### 04_architecture 架构

| 文件 | 说明 |
|---|---|
| SYSTEM_ARCH.md | 技术架构：总体架构图 / 选型理由 / 关键质量决策 / 部署运行 / 风险边界 |
| MODULE_ARCH.md | 模块拆分：新仓目标结构 / 公共能力归位 / 规划模块预留 / 依赖规则 / 迁移对照 |
| STATE_MACHINE.md | 状态管理：状态全景归属 / 五态状态机与不变式 / stores 设计 / 事件→状态映射总表 |
| DATA_FLOW.md | 数据流动：端到端流图 / 连接配置→加密存储→SSH 会话三条链路 / 密钥绑定 hostname+username 事实（P8 补齐） |

### 05_sequence 时序（P8 补齐）

| 文件 | 说明 |
|---|---|
| SEQUENCE_DIAGRAMS.md | 6 张 sequenceDiagram：新建连接+认证 / 会话 IO 循环 / 断线与错误 / 手动重连 / 凭据加解密 / 关闭会话（现状与 V1 修复后规格 Note 区分） |

### 06_review 评审

| 文件 | 说明 |
|---|---|
| PRODUCT_REVIEW.md | P4 产品体验审查：A2/B13/C8/D7 四类分级，公共能力识别四类归位表 |
| UX_REVIEW.md | UX 综合评审：P4 + UF 分册合并全景 / 流程健康度 / 画像短板 / 修复优先级（P8 补齐） |
| IA_REVIEW.md | IA 综合评审：信息架构现状 / 逐页归属 / 可预测性 29% / IA-01..08 清单（P8 补齐） |

### 07_design_system 设计系统

| 文件 | 说明 |
|---|---|
| TOKEN.md | 设计令牌：色板（Tokyo Night）/ 字体 / 间距 / 圆角 / 布局 / 动效 / z-index / 公共参数（五态、快捷键、TOFU、加密参数） |
| COMPONENT.md | 组件：HostCard / StatusDot / Tab 族 / AuthForm / KeyFingerprintConfirm / CommandPalette / Toast / 布局件 + §9 组件记录表（18 项） |
| PATTERN.md | 交互模式：连接-会话-中断流转 / 危险确认 / 快捷键 / 空态 / 反馈三级 / 表单 / 列表选择 / 布局 / 安全交互 |
| ASSETS.md | 图标资产：旧项目 6 枚内联 SVG 逐字收录 + V1 新增 8 枚 + 使用规范 + 资产边界 |
| GUIDELINES.md | 使用指南：总原则 / 命名约定 / 可用性基线 / 文案规范 / 禁止事项 / 评审变更流程 |

### 08_development 开发

| 文件 | 说明 |
|---|---|
| DATA_MODEL.md | 数据模型：ER 图 / 已实现实体字段级 / 规划实体预留 / 所有权生命周期 |
| API_SPEC.md | API 契约：8+2 命令逐条契约 / 错误码归类 / 事件契约规则 / 关键时序 |
| ERROR_CODE.md | 错误处理盘点（8 前端 catch 点 + 5 后端静默/降级点）+ 错误码规范建议 E_*（P8 补齐） |
| PERMISSION.md | 权限规格：系统权限 S1-S7 / 应用内 A1-A6 / 加密密钥体系 / known_hosts / 目标态（P8 补齐） |

### 09_test 测试

| 文件 | 说明 |
|---|---|
| COVERAGE_CHECKLIST.md | HTML 覆盖检查表（对 V0 原型）：页面 10 / 功能 F001-F046 / 六项特检全覆盖 |
| HTML_V0_ACCEPTANCE.md | V0 原型验收报告：T1-T20 浏览器用例 + D1-D4 缺陷修复，结论 PASS |
| V1_ACCEPTANCE.md | V1 原型验收报告：浏览器 + jsdom 35+ 断言 + D1-D7 缺陷修复，46/46 覆盖，结论 PASS |

### product-review 产品逻辑评审六件套（目录位置不动，2026-09-03 评审产出）

| 文件 | 说明 |
|---|---|
| PRODUCT_LOGIC_REVIEW.md | 总报告：§17 八项验收逐项结论（0 达标/4 部分/4 不达标）+ 43 项问题汇总 + 取舍四档 + Top5 发现 |
| INFORMATION_ARCHITECTURE_REVIEW.md | IA 分册：导航层级 / 逐页归属 / 可预测性抽查 / IA-01..08 |
| USER_FLOW_REVIEW.md | UF 分册：8 流程五要素核查 / 异常三类核查 / UF-01..09 |
| DATA_STORAGE_REVIEW.md | DS 分册：D1-D11 数据清单 / 加解密链路核实 / DS-01..09 |
| STATE_REVIEW.md | ST 分册：4 套状态体系 / 五态状态机实测 / 双轨对照 / ST-01..07 |
| PERMISSION_REVIEW.md | PM 分册：系统权限 S1-S7 / 应用内 A1-A6 / PM-01..05 |

### prototype 原型（原位不动）

| 文件 | 说明 |
|---|---|
| v0-old/app-prototype.html | V0 旧版事实基线原型（只读；验收见 09_test/HTML_V0_ACCEPTANCE.md） |
| v1-new/app-prototype.html | V1 新版原型（B 类 13 项优化落地；验收见 09_test/V1_ACCEPTANCE.md） |

## 三、推荐阅读顺序

1. **入门**：`00_context/PROJECT_CONTEXT.md` → `00_context/TECH_STACK.md`（5 分钟了解项目与栈）
2. **事实基线**：`01_reverse/REVERSE_ANALYSIS.md`（一切结论的根文档）
3. **产品层**：`02_product/PRODUCT_MODEL.md` → `02_product/PRD.md` → `02_product/FEATURE_MAP.md` → `02_product/PAGE_SPEC.md`
4. **流程层**：`03_flow/USER_FLOW.md` → `03_flow/PAGE_FLOW.md` → `03_flow/BUSINESS_FLOW.md`
5. **评审层**：`06_review/PRODUCT_REVIEW.md`（P4）→ `product-review/PRODUCT_LOGIC_REVIEW.md`（总）→ 五分册按需 → `06_review/UX_REVIEW.md` / `06_review/IA_REVIEW.md`（综合）
6. **设计层**：`07_design_system/GUIDELINES.md` → `TOKEN.md` → `COMPONENT.md` → `PATTERN.md` → `ASSETS.md`
7. **开发层**：`04_architecture/SYSTEM_ARCH.md` → `04_architecture/MODULE_ARCH.md` → `04_architecture/STATE_MACHINE.md` → `04_architecture/DATA_FLOW.md` → `05_sequence/SEQUENCE_DIAGRAMS.md` → `08_development/DATA_MODEL.md` → `08_development/API_SPEC.md` → `08_development/ERROR_CODE.md` → `08_development/PERMISSION.md`
8. **测试层**：`09_test/COVERAGE_CHECKLIST.md` → `09_test/HTML_V0_ACCEPTANCE.md` → `09_test/V1_ACCEPTANCE.md`
9. **原型对照**：`prototype/v0-old/`（旧基线）vs `prototype/v1-new/`（体验增量）

## 四、旧 → 新路径映射表（2026-09-03 迁移）

| 旧路径 | 新路径 |
|---|---|
| docs/reverse-analysis.md | docs/01_reverse/REVERSE_ANALYSIS.md |
| docs/product/prd.md | docs/02_product/PRD.md |
| docs/product/page-spec.md | docs/02_product/PAGE_SPEC.md |
| docs/product/html-coverage-checklist.md | docs/09_test/COVERAGE_CHECKLIST.md |
| docs/product/html-acceptance-report.md | docs/09_test/HTML_V0_ACCEPTANCE.md |
| docs/review/product-review.md | docs/06_review/PRODUCT_REVIEW.md |
| docs/review/v1-acceptance.md | docs/09_test/V1_ACCEPTANCE.md |
| docs/architecture/tech-architecture.md | docs/04_architecture/SYSTEM_ARCH.md |
| docs/architecture/module-split.md | docs/04_architecture/MODULE_ARCH.md |
| docs/architecture/state-management.md | docs/04_architecture/STATE_MACHINE.md |
| docs/architecture/data-model.md | docs/08_development/DATA_MODEL.md |
| docs/architecture/api-design.md | docs/08_development/API_SPEC.md |
| design-system/tokens.md | docs/07_design_system/TOKEN.md |
| design-system/components.md | docs/07_design_system/COMPONENT.md |
| design-system/patterns.md | docs/07_design_system/PATTERN.md |
| design-system/assets.md | docs/07_design_system/ASSETS.md |
| design-system/guidelines.md | docs/07_design_system/GUIDELINES.md |

迁移说明：
- 上述 17 处移动后，`docs/product/`、`docs/architecture/`、`design-system/` 目录已空并删除；`docs/review/` 因留存验收截图 `termforge-v1-final.png` 保留。
- `docs/product-review/` 六件套按 SOP 要求**目录位置不动**，仅其内部引用的旧文档路径已同步更新。
- `prototype/{v0-old,v1-new}/` 原位不动。
- 本批同时补齐 16 个新文件（上表标注"P8 补齐"）并建立本索引。

# TermForge 资产清单（ASSET INVENTORY）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 范围：本会话重构产物资产（文档/设计系统/原型/截图）+ 项目既有资产（图标/文档站/过程资产）。逐项以 2026-09-03 `ls`/`find` 实测为准；内容性质引自对应文档。

---

## 1. 本套重构产物资产（编号文档体系 + 原型）

### 1.1 docs/ 编号目录（00_context～09_test + 索引）

| 资产 | 位置 | 性质 | 来源/说明 |
|---|---|---|---|
| 逆向分析报告 | `docs/01_reverse/REVERSE_ANALYSIS.md` | 事实基线文档 | 逐文件阅读 src-ui/src-tauri 全部源码 + 既有文档交叉对照生成（P1） |
| PRD | `docs/02_product/PRD.md` | 产品需求 | 由旧代码逆向 + 既有规划对照生成，F001-F046（P2） |
| 页面交互规格 | `docs/02_product/PAGE_SPEC.md` | 交互规格 | 10 页面 11 维度 + 六项特检矩阵（P3） |
| 产品体验审查报告 | `docs/06_review/PRODUCT_REVIEW.md` | 审查报告 | A/B/C/D 四类分级，B 类 13 项（P4） |
| 设计系统五文件 | `docs/07_design_system/{TOKEN,COMPONENT,PATTERN,ASSETS,GUIDELINES}.md` | 设计系统 | 令牌/组件/交互模式/图标/使用指南，全部自旧项目真实样式提取并注明来源（P5） |
| 开发架构四文件 | `docs/04_architecture/{SYSTEM_ARCH,MODULE_ARCH,STATE_MACHINE}.md` + `docs/08_development/{DATA_MODEL,API_SPEC}.md` | 开发规格 | 技术架构/模块拆分/状态管理/数据模型/API 契约（P7） |
| 覆盖检查表 | `docs/09_test/COVERAGE_CHECKLIST.md` | 测试文档 | 页面/功能/特检覆盖核对（对 V0 原型负责） |
| V0 原型验收报告 | `docs/09_test/HTML_V0_ACCEPTANCE.md` | 测试文档 | 20 组浏览器用例 + 4 项缺陷修复记录，结论 PASS |
| V1 原型验收报告 | `docs/09_test/V1_ACCEPTANCE.md` | 测试文档 | 浏览器 + jsdom 双环境 35+ 断言 + 7 项缺陷修复记录，结论 PASS |
| 产品逻辑评审六件套 | `docs/product-review/*.md`（6 个文件，目录位置不动） | 评审报告 | 总报告 + IA/UF/DS/ST/PM 五分册，43 项问题（A2/B20/C7/D14） |
| 上下文四文件 | `docs/00_context/{PROJECT_CONTEXT,TECH_STACK,ASSET_INVENTORY,DEPENDENCY_LIST}.md` | 上下文 | 本批补齐（P8） |
| 流程三文件 | `docs/03_flow/{USER_FLOW,PAGE_FLOW,BUSINESS_FLOW}.md` | 流程 | 本批补齐（P8） |
| 数据流动 | `docs/04_architecture/DATA_FLOW.md` | 架构 | 本批补齐（P8） |
| 时序图集 | `docs/05_sequence/SEQUENCE_DIAGRAMS.md` | 架构 | 本批补齐（P8） |
| 综合评审二文件 | `docs/06_review/{UX_REVIEW,IA_REVIEW}.md` | 评审 | 本批补齐（P8，综合 P4 与六件套） |
| 开发规范二文件 | `docs/08_development/{ERROR_CODE,PERMISSION}.md` | 规范 | 本批补齐（P8） |
| 文档索引 | `docs/DOCUMENT_INDEX.md` | 索引 | 目录树 + 阅读顺序 + 旧→新路径映射（本批补齐） |

### 1.2 prototype/（原位不动）

| 资产 | 位置 | 性质 | 说明 |
|---|---|---|---|
| V0 旧版原型 | `prototype/v0-old/app-prototype.html` | 单文件可交互 HTML 原型（只读基线） | 旧项目事实基线快照；零外链、内联 SVG；验收见 09_test/HTML_V0_ACCEPTANCE.md |
| V1 新版原型 | `prototype/v1-new/app-prototype.html` | 单文件可交互 HTML 原型 | 含 P4 审查 13 项 B 类优化落地；验收见 09_test/V1_ACCEPTANCE.md |

### 1.3 验收截图

| 资产 | 位置 | 说明 |
|---|---|---|
| V1 验收截图 | `docs/review/termforge-v1-final.png` | Playwright 生成，随 V1 验收入库（吸取 V0 截图未入库教训）；被 `docs/09_test/V1_ACCEPTANCE.md` 引用。目录为遗留位置（review/ 其余文件已迁走，仅留此图） |
| V0 验收截图 | 不存在 | HTML_V0_ACCEPTANCE.md A-2 勘误：`termforge-prototype-final.png` 未随仓库留存，全仓检索无此文件 |

## 2. 项目既有资产（非本套产物，仅登记不改动）

### 2.1 应用图标（`src-tauri/icons/`）

| 文件 | 用途 |
|---|---|
| icon.icns / icon.ico / icon.png / 32x32.png / 128x128.png / 128x128@2x.png | 应用安装包/窗口图标（tauri.conf.json bundle.icon 引用）。**仅为安装包图标，非 UI 内使用**（来源：docs/07_design_system/ASSETS.md §4：旧项目无位图资产依赖；UI 图标全部为 icons.ts 内联 SVG） |

### 2.2 文档站资产（`docs/`，项目自有）

| 资产 | 位置 | 说明 |
|---|---|---|
| VitePress 站点 | `docs/index.md`、`docs/.vitepress/config.mjs`、`docs/package.json`、`docs/public/CNAME`、`docs/node_modules/`、`docs/.vitepress/dist/` | 项目自有文档站（GitHub Pages 部署，workflow `.github/workflows/deploy-docs.yml`）。**项目自有文件，本套迁移不改动** |

### 2.3 过程资产（BMAD 方法论）

| 资产 | 位置 | 说明 |
|---|---|---|
| BMAD 配置 | `_bmad/`、`.claude/skills/` | 开发流程资产，非产品代码（REVERSE_ANALYSIS §②） |
| 规划产物 | `_bmad-output/planning-artifacts/`（prd.md / architecture.md / epics.md / ux-design-specification.md / readiness-report） | 旧规划文档；sprint-status.yaml 已过时（见 PROJECT_CONTEXT.md §5），不能作为实现状态依据 |
| 实施产物 | `_bmad-output/implementation-artifacts/`（sprint-status.yaml / deferred-work.md / story 1-1~1-6） | 同上；deferred-work.md 记录的已知缺陷（closeTab 泄漏等）已被本套评审体系覆盖 |
| 旧版项目文档 | README.md / CLAUDE.md / PROGRESS.md / 产品需求.md / 代码骨架.md | 项目自有；存在 Terminal.svelte 死链与时间线矛盾（PROJECT_CONTEXT.md §5） |

### 2.4 前端源码资产（`src-ui/src/`，事实基线，不改动）

- 16 个源文件：App.svelte、main.ts、app.css、4 layout、3 primitives、ConnectionForm、TerminalTab、4 lib（api/types/toast/icons）；`stores/` 目录存在但为空（2026-09-03 实测）。
- 设计令牌单源：`app.css` :root（Tokyo Night 色板）——已全量收录至 `docs/07_design_system/TOKEN.md`。

### 2.5 后端源码资产（`src-tauri/src/`，事实基线，不改动）

- 10 个源文件：lib.rs、main.rs、commands/{mod,session,store}.rs、core/{mod,crypto,session_manager}.rs、core/ssh/{mod,client}.rs、models/{mod,dto,events}.rs。

### 2.6 运行时数据资产（用户机器上，应用自建）

| 资产 | 位置 | 说明 |
|---|---|---|
| 连接库 | `~/.termforge/connections.json` | `{connections:[{id,name,host,port,username,password(加密base64)}]}`，Unix 0600（REVERSE_ANALYSIS §⑦） |
| 已知主机 | `~/.termforge/known_hosts` | 每行 `host:port hex指纹`，TOFU 首录，Unix 0600（注意：指纹实为主机公钥 raw hex 非哈希，见 docs/product-review/PRODUCT_LOGIC_REVIEW.md PL-02 勘误） |

## 3. 资产健康度备注

1. **V0/V1 原型均为单文件零外链**（无 CDN/网络字体/外链图片；图标内联 SVG）——可在 file:// 直开（各验收报告非阻断备注）。
2. **UI 内无位图资产**；Toast 图标为文本符号 ✓/✕/ℹ（事实保留，ASSETS.md §3）。
3. **无 favicon**（HTTP 访问时浏览器请求 404，file:// 不受影响——两份验收报告非阻断备注 1）。
4. 旧项目文档（README 等 5 份）存在死链与过期描述——阅读顺序以 `docs/DOCUMENT_INDEX.md` 为准，勿直接采信旧文档（PROJECT_CONTEXT.md §5）。

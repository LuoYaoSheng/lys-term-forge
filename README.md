# TermForge

> `lys: 跨平台 SSH / SFTP / Runbook 运维工作台`
>
> Part of the lys personal open source system.

`TermForge` 不是单纯的 SSH 客户端，而是一个面向开发者和运维场景的桌面工作台。  
规划中的公开仓库名为 `lys-term-forge`。

---

## 当前状态

当前本地项目已经有真实工程骨架，不能再按纯规划项目来描述。

当前更准确的描述是：

- 已有 Tauri 2 + Rust + Svelte 工作区
- 已有终端组件、命令层和会话管理基础
- 仍处于早期阶段，主链路还在打磨

---

## 当前技术栈

- Tauri 2
- Rust
- Svelte
- Vite
- xterm.js
- `ssh2`

注意：

- 当前文档和代码现实应按 Svelte 路线书写
- 不再默认把 React 方案当成当前实现

---

## 产品目标

`TermForge` 想解决的不是“再造一个终端”，而是把常见运维动作收进一个桌面工作台：

- 连接中心
- SSH 终端
- SFTP 文件管理
- 端口转发
- Runbook 批量执行
- 本地安全存储

---

## 当前代码结构

```text
TermForge/
├── src-ui/
│   ├── src/
│   │   ├── components/
│   │   │   ├── Terminal.svelte
│   │   │   └── TerminalTab.svelte
│   │   ├── lib/api.ts
│   │   └── App.svelte
│   └── package.json
└── src-tauri/
    ├── src/
    │   ├── commands/
    │   ├── core/
    │   └── models/
    └── Cargo.toml
```

---

## 文档入口

- [产品需求](./产品需求.md)
- [代码骨架](./代码骨架.md)
- [开发进度](./PROGRESS.md)

---

## 近期重点

1. 修当前终端显示链路
2. 收口连接与会话主流程
3. 再推进 SFTP、端口转发和 Runbook
4. 达到公开条件后，再考虑进入 Gitee 主线

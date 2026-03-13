# nanoaios 路线图（简洁优先）

## v0.1（当前）

- 单二进制内核
- `init/start/chat/config` 基础命令
- Runtime Provider 抽象（mock + openai_compatible）
- 最小 API 可观测面

## v0.2（建议）

- Session 与 Memory 子系统（先文件，再 SQLite）
- Tool 执行层（白名单 + timeout + 审计日志）
- Agent manifest（能力声明 + 权限门控）

## v0.3（建议）

- Daemon 模式（pid, log, restart）
- 插件协议（MCP 优先）
- 多 provider 路由（fallback + 成本感知）

## 非目标（暂不做）

- 大量预置“功能包”导致内核膨胀
- 多层脚手架与复杂配置矩阵
- 为兼容历史生态而牺牲内核简洁性

核心原则：**先把“AIOS Linux 内核面”做实，再扩展用户态生态。**

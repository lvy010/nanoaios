# nanoaios 路线图（简洁优先）

## v0.1（已完成）

- 单二进制内核
- `init/start/chat/config` 基础命令
- Runtime Provider 抽象（mock + openai_compatible）
- 最小 API 可观测面

## v0.2（当前）

- Session 与 Memory 子系统（文件型最小实现）
- Tool 执行层（TOML manifest 注册、白名单、超时保护）
- `tool list / add / remove` CLI 命令
- `/v1/tools` 与 `/v1/tools/{name}/invoke` API 端点
- HTTP Tool 调用（GET / POST）

## v0.3（计划中）

- MCP Client 协议（连接任意 MCP Server）
- Agent manifest（能力声明 + 权限门控）
- Daemon 模式（pid, log, restart）
- 多 provider 路由（fallback + 成本感知）
- Tool 调用审计日志

## 非目标（暂不做）

- 大量预置"功能包"导致内核膨胀
- 多层脚手架与复杂配置矩阵
- 为兼容历史生态而牺牲内核简洁性

核心原则：**先把"AIOS Linux 内核面"做实，再扩展用户态生态。**

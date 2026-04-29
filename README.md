# nanoaios

[记录了当时的一些想法切片](https://lvyovo-wiki.tech/blog/memo-claw)

`nanoaios` ：一个面向 Agent 时代的极简 AIOS-kernel demo实现，定位为 **AIOS 的 Linux**。  

目标不是堆叠“AI 功能集合”，而是提供一套可验证、可扩展、可替换的系统级 AI 抽象。

~~快速 vibe 的一个项目~~，之后或许会继续完善，欢迎交流讨论


---

## 为什么是现在

第二次 API 开放浪潮正在到来。

2025 下半年，大模型达到生产级临界点，AI 的核心价值从"内容生成"转向"内容生成 + 自动化"。自动化意味着 Agent 必须能调用外部平台——而外部平台必须先开放 API。MCP、Skill 等协议正在成为这波开放的标准接口。

没有 API 的平台将被 Agent 工作流绕过；有 API 的平台则需要一个**系统级的调度内核**来安全、可控地代替用户行事。

`nanoaios` 就是这个内核——不堆功能，只做推理调度、Tool 编排、权限管控和生命周期管理。

---

## 项目定位

<p align="center"><img src="png/nanoaios.png" alt="nanoaios" /></p>

在 `nanoaios` 中，AI 不是外挂能力，而是系统原生能力：

- 用最小内核面承载推理入口与系统状态
- 用运行时抽象屏蔽模型供应商差异
- 用 Tool 层对接外部平台 API（MCP / HTTP）
- 用统一 API 暴露可观测与可集成能力

核心原则：

- **简洁优先**：单仓库、单二进制、低复杂度
- **边界清晰**：Kernel / Runtime / Tool / Provider 分层明确
- **开放接入**：通过 Tool manifest 对接任意 MCP Server 或 HTTP API
- **工程可落地**：可启动、可测试、可演进

---

## 当前能力（v0.2 alpha）

- `nanoaios init`：初始化 `~/.nanoaios/config.toml`
- `nanoaios start`：启动内核 API
- `nanoaios chat "<prompt>"`：单轮推理调用
- `nanoaios chat "<prompt>" --session <id>`：带 Session 的推理调用（自动写入本地记忆）
- `nanoaios session <id>`：查看某个 Session 的本地记忆
- `nanoaios config`：打印当前配置
- `nanoaios tool list`：列出已注册的 Tool
- `nanoaios tool add <manifest.toml>`：注册新 Tool
- `nanoaios tool remove <tool_name>`：移除 Tool
- Provider 抽象：
  - `mock`（离线调试）
  - `openai_compatible`（OpenAI 兼容接口）
- Session / Memory（文件型最小实现）：
  - 默认存储在 `~/.nanoaios/sessions/`
  - 支持按 `session_id` 读取历史
  - 支持 `max_messages_per_session` 上限裁剪
- Tool 执行层：
  - 通过 TOML manifest 注册外部 Tool（MCP / HTTP）
  - 白名单管控 + 超时保护
  - `~/.nanoaios/tools/` 目录管理 Tool manifest

已提供 API：

- `GET /healthz`
- `GET /v1/kernel/state`
- `GET /v1/kernel/memory/{session_id}`
- `POST /v1/chat/completions`
- `GET  /v1/tools`
- `POST /v1/tools/{tool_name}/invoke`

---

## 快速开始

### 1) 初始化

```bash
cd nanoaios
cargo run -- init
```

### 2) 启动服务

```bash
cargo run -- start
```

### 3) 验证服务

```bash
curl -s http://127.0.0.1:4242/
curl -s http://127.0.0.1:4242/healthz
curl -s http://127.0.0.1:4242/v1/kernel/state
```

浏览器直接访问 `http://localhost:4242/` 也会返回服务入口信息（JSON）。

### 4) 对话测试

```bash
cargo run -- chat "你好，nanoaios"
```

带 Session 的记忆对话：

```bash
cargo run -- chat "我叫小明" --session demo01
cargo run -- chat "你还记得我叫什么吗？" --session demo01
cargo run -- session demo01
```

---

### 5) Tool 管理

注册一个 Tool（以 HTTP API 为例）：

```bash
cargo run -- tool add examples/tools/weather.toml
cargo run -- tool list
```

`weather.toml` manifest 示例：

```toml
name = "weather"
description = "查询城市天气"
kind = "http"
base_url = "https://api.weather.example.com"
method = "GET"
path = "/v1/current"
timeout_secs = 10
```

通过 API 调用已注册的 Tool：

```bash
curl -X POST http://127.0.0.1:4242/v1/tools/weather/invoke \
  -H 'Content-Type: application/json' \
  -d '{"params": {"city": "Shanghai"}}'
```

---

## 架构概览

`nanoaios` 采用最小分层：

- **CLI**：命令入口与系统操作面
- **Config**：配置声明与加载
- **Kernel**：系统状态与推理调度入口
- **Runtime**：调用生命周期与错误处理
- **Tool**：外部 API 注册、发现与调用（MCP / HTTP）
- **Provider**：模型供应商适配层
- **API**：可观测、可集成接口

目录结构：

```text
src/
  main.rs
  cli.rs
  config.rs
  kernel.rs
  runtime.rs
  api.rs
  tool.rs
  memory.rs
docs/
  TESTING.md
  ARCHITECTURE.md
  ROADMAP.md
```

---

## 开发与验证

```bash
# 代码格式检查
cargo fmt -- --check

# 严格静态检查
cargo clippy -- -D warnings

# 测试
cargo test

# 冒烟验证
cargo run -- init --force
cargo run -- chat "smoke test"
cargo run -- chat "remember me" --session smoke01
cargo run -- session smoke01
```

---

## Session / Memory 配置

默认配置新增了 `memory` 段：

```toml
[memory]
enabled = true
max_messages_per_session = 50
```

说明：

- `enabled = true`：启用本地文件记忆
- `max_messages_per_session`：每个会话保留的最大消息条数（超过会自动裁剪旧消息）

更完整步骤见 `docs/TESTING.md`。

---

## 路线图

短期目标（v0.2 / v0.3）：

- ~~Session / Memory 子系统~~ ✅
- ~~Tool 执行层（manifest 注册、白名单、超时）~~ ✅
- Agent manifest 与能力门控
- MCP Client 协议（连接任意 MCP Server）
- Daemon 化与服务管理

详见 `docs/ROADMAP.md`。

---

## 稳定性说明

当前版本处于早期阶段，接口和内部实现仍会迭代。  
建议在生产环境固定 commit 或版本后再部署。

---

## 贡献

欢迎提交 Issue / PR。建议在提交前执行：

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

详细贡献说明见 `CONTRIBUTING.md`。

---

## 许可协议

本项目采用 MIT License，详见 `LICENSE`。

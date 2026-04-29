# nanoaios

> 面向 Agent 时代的极简 AIOS 内核，定位为 **AIOS 的 Linux**。

不堆叠功能集合，只提供可验证、可扩展、可替换的系统级 AI 抽象。

## 背景

第二次 API 开放浪潮正在到来。大模型达到生产级临界点后，AI 的核心价值从内容生成转向 **内容生成 + 自动化**。Agent 必须能调用外部平台，而外部平台必须先开放 API——MCP、Skill 等协议正在成为标准接口。

有 API 的平台需要一个系统级调度内核来安全、可控地代替用户行事。`nanoaios` 就是这个内核：推理调度、Tool 编排、权限管控、生命周期管理。

> [更多思考](https://lvyovo-wiki.tech/blog/memo-claw)

## 架构

```text
┌─────────────────────────────────────┐
│               CLI                   │  命令入口
├─────────────────────────────────────┤
│               API                   │  HTTP 接口（可观测、可集成）
├──────────┬──────────┬───────────────┤
│  Kernel  │  Memory  │    Tool       │  状态调度 / 会话记忆 / 外部 API
├──────────┴──────────┴───────────────┤
│             Runtime                 │  调用生命周期与错误处理
├─────────────────────────────────────┤
│             Provider                │  模型供应商适配（mock / openai）
└─────────────────────────────────────┘
```

核心原则：**简洁优先** · **边界清晰** · **开放接入** · **工程可落地**

## 目录结构

```text
src/
├── main.rs          # 入口
├── cli.rs           # 命令定义
├── config.rs        # 配置加载
├── kernel.rs        # 内核状态与调度
├── runtime.rs       # 推理运行时
├── memory.rs        # Session / Memory
├── tool.rs          # Tool 注册与调用
└── api.rs           # HTTP API
examples/
└── tools/           # Tool manifest 示例
docs/
├── ARCHITECTURE.md
├── ROADMAP.md
└── TESTING.md
```

## 快速开始

```bash
git clone https://github.com/lvy010/nanoaios.git
cd nanoaios

# 初始化配置
cargo run -- init

# 启动服务
cargo run -- start
```

验证：

```bash
curl -s http://127.0.0.1:4242/healthz
curl -s http://127.0.0.1:4242/v1/kernel/state
```

## 使用

### 对话

```bash
cargo run -- chat "你好，nanoaios"
```

带 Session 记忆：

```bash
cargo run -- chat "我叫小明" --session demo01
cargo run -- chat "你还记得我叫什么吗？" --session demo01
cargo run -- session demo01
```

### Tool 管理

注册 Tool：

```bash
cargo run -- tool add examples/tools/weather.toml
cargo run -- tool list
cargo run -- tool remove weather
```

Tool manifest 格式（TOML）：

```toml
name = "weather"
description = "查询城市天气"
kind = "http"
base_url = "https://api.weather.example.com"
method = "GET"
path = "/v1/current"
timeout_secs = 10
```

通过 API 调用 Tool：

```bash
curl -X POST http://127.0.0.1:4242/v1/tools/weather/invoke \
  -H 'Content-Type: application/json' \
  -d '{"params": {"city": "Shanghai"}}'
```

## API

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/healthz` | 健康检查 |
| `GET` | `/v1/kernel/state` | 内核状态 |
| `GET` | `/v1/kernel/memory/{session_id}` | 会话记忆 |
| `POST` | `/v1/chat/completions` | 推理调用 |
| `GET` | `/v1/tools` | 已注册 Tool 列表 |
| `POST` | `/v1/tools/{tool_name}/invoke` | 调用指定 Tool |

## 配置

初始化后配置文件位于 `~/.nanoaios/config.toml`：

```toml
node_name = "nanoaios-local"
api_host = "127.0.0.1"
api_port = 4242

[provider]
kind = "mock"
model = "nanoaios/mock-v1"
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"

[memory]
enabled = true
max_messages_per_session = 50
```

Tool manifest 存储在 `~/.nanoaios/tools/` 目录。

## 开发

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

冒烟验证：

```bash
cargo run -- init --force
cargo run -- chat "smoke test"
cargo run -- tool list
```

## 路线图

| 版本 | 状态 | 内容 |
|------|------|------|
| v0.1 | ✅ | 单二进制内核、CLI、Provider 抽象、最小 API |
| v0.2 | ✅ | Session/Memory、Tool 执行层（HTTP）、Tool CLI & API |
| v0.3 | 计划 | MCP Client 协议、Agent manifest、Daemon 模式 |

详见 [`docs/ROADMAP.md`](docs/ROADMAP.md)。

## 贡献

欢迎提交 Issue / PR，提交前请执行 `cargo fmt --check && cargo clippy -- -D warnings && cargo test`。

详见 [`CONTRIBUTING.md`](CONTRIBUTING.md)。

## 许可协议

[MIT](LICENSE)


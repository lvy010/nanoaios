# nanoaios 测试与启动指南

本文档目标：让用户在 10 分钟内完成从初始化到端到端验证。

## 1. 环境要求

- Linux / macOS（本文以 Linux 为例）
- Rust 工具链（建议 stable）
- 网络可选（使用 `mock` provider 时可离线）

检查命令：

```bash
rustc --version
cargo --version
```

## 2. 初始化配置

```bash
cd nanoaios
cargo run -- init
```

预期输出类似：

```text
配置文件已就绪: /home/<you>/.nanoaios/config.toml
```

查看配置：

```bash
cargo run -- config
```

默认是 `mock` provider，可直接离线测试。

## 3. 启动内核 API

```bash
cargo run -- start
```

预期输出类似：

```text
nanoaios api running on http://127.0.0.1:4242
```

进程会持续运行，这就是最小内核常驻形态。

## 4. 健康检查与状态验证

另开一个终端执行：

```bash
curl -s http://127.0.0.1:4242/
curl -s http://127.0.0.1:4242/healthz
```

预期返回：

```json
{"ok":true,"service":"nanoaios"}
```

继续检查状态：

```bash
curl -s http://127.0.0.1:4242/v1/kernel/state | jq
```

预期包含：

- `node_name`
- `provider_model`
- `boot_unix_ms`
- `turns`

## 5. 单轮推理测试（mock）

```bash
cargo run -- chat "帮我验证 nanoaios 的最小闭环"
```

预期输出：

```text
mock-response: 帮我验证 nanoaios 的最小闭环
```

这说明 Runtime 与 Provider 抽象链路可用。

## 6. 切换 OpenAI 兼容 Provider（可选）

编辑 `~/.nanoaios/config.toml`：

```toml
[provider]
kind = "openai_compatible"
model = "gpt-4o-mini"
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"
```

设置密钥并测试：

```bash
export OPENAI_API_KEY="<your_key>"
cargo run -- chat "用一句话介绍 nanoaios"
```

## 7. 常见问题

### Q1: `读取配置失败`

先执行：

```bash
cargo run -- init --force
```

### Q2: 端口冲突（4242）

修改 `~/.nanoaios/config.toml` 的 `api_port`，然后重启 `start`。

### Q3: OpenAI 请求失败

检查：

- `OPENAI_API_KEY` 是否已导出
- `base_url` 是否正确
- `model` 是否可用

## 8. 回归测试清单（每次改动后）

- [ ] `cargo fmt -- --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo run -- init --force`
- [ ] `cargo run -- start` + `curl /` + `curl /healthz`
- [ ] `cargo run -- chat "smoke test"`

如果以上通过，说明最小系统仍保持可启动、可观测、可调用。

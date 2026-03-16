# nanoaios 测试与启动指南

本文档目标：让用户在 10 分钟内完成从初始化到端到端验证。

## 1. 环境要求

- Linux / macOS
- Rust 工具链（建议 stable）
- 网络可选（使用 `mock` provider 时可离线）
- Docker（可选，Docker 部署时需要）

检查命令：

```bash
rustc --version
cargo --version
docker --version   # 可选
```

## 2. 配置

配置优先级：环境变量 > config.toml > 默认值

### 方式一：.env 文件（推荐）

```bash
cd nanoaios
cp .env.example .env
```

编辑 `.env`，按需修改：

```env
NANOAIOS_API_HOST=0.0.0.0
NANOAIOS_PROVIDER_KIND=mock
OPENAI_API_KEY=sk-your-api-key-here
```

### 方式二：config.toml

```bash
cargo run -- init
cargo run -- config
```

默认是 `mock` provider，可直接离线测试。

## 3. 启动内核 API

```bash
# 本地
cargo run -- start

# Docker
docker compose up -d --build
```

预期输出类似：

```text
nanoaios api running on http://0.0.0.0:4242
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

浏览器直接访问 `http://127.0.0.1:4242/` 也会返回服务入口信息（JSON）。

## 5. 单轮推理测试（mock）

```bash
# 本地
cargo run -- chat "帮我验证 nanoaios 的最小闭环"

# Docker
docker compose exec -it nanoaios nanoaios chat "帮我验证 nanoaios 的最小闭环"
```

预期输出：

```text
mock-response: 帮我验证 nanoaios 的最小闭环
```

这说明 Runtime 与 Provider 抽象链路可用。

## 6. 切换 OpenAI 兼容 Provider（可选）

编辑 `.env`：

```env
NANOAIOS_PROVIDER_KIND=openai_compatible
NANOAIOS_PROVIDER_MODEL=gpt-4o-mini
NANOAIOS_PROVIDER_BASE_URL=https://api.openai.com/v1
OPENAI_API_KEY=<your_key>
```

本地直接测试，Docker 执行 `docker compose up -d` 重新加载：

```bash
cargo run -- chat "用一句话介绍 nanoaios"
```

也可通过编辑 `~/.nanoaios/config.toml` 切换：

```toml
[provider]
kind = "openai_compatible"
model = "gpt-4o-mini"
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"
```

```bash
export OPENAI_API_KEY="<your_key>"
cargo run -- chat "用一句话介绍 nanoaios"
```

## 7. 常见问题

### Q1: 读取配置失败

方式一：确认已 `cp .env.example .env` 并正确填写。

方式二：执行 `cargo run -- init --force`。

### Q2: 端口冲突（4242）

修改 `.env` 的 `NANOAIOS_API_PORT`，或修改 `~/.nanoaios/config.toml` 的 `api_port`，然后重启。

### Q3: Docker 下 curl 无响应

确认 `.env` 中 `NANOAIOS_API_HOST=0.0.0.0`，容器内 `127.0.0.1` 对宿主机不可达。

### Q4: OpenAI 请求失败

检查：

- `OPENAI_API_KEY` 是否已设置
- `NANOAIOS_PROVIDER_BASE_URL` 是否正确
- `NANOAIOS_PROVIDER_MODEL` 是否可用

## 8. 回归测试清单（每次改动后）

- [ ] `cargo fmt -- --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo run -- start` + `curl /` + `curl /healthz`
- [ ] `cargo run -- chat "smoke test"`
- [ ] `docker compose up -d --build` + `curl /healthz`
- [ ] `docker compose exec -it nanoaios nanoaios chat "smoke test"`

如果以上通过，说明最小系统仍保持可启动、可观测、可调用。

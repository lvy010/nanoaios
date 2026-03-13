# nanoaios 架构说明

## 设计目标

把 `nanoaios` 做成 “AIOS 的 Linux”，强调：

- 原生 AI 内核能力
- 小而清晰的系统抽象
- 可验证、可演进、可替换

## 分层

1. **CLI 层**（`main.rs`, `cli.rs`）
   - 系统入口与命令分发
2. **Config 层**（`config.rs`）
   - 运行参数与 provider 声明
3. **Kernel 层**（`kernel.rs`）
   - 统一系统状态与推理入口
4. **Runtime 层**（`runtime.rs`）
   - 调用链路、模型请求、错误处理
5. **API 层**（`api.rs`）
   - 最小可观测接口，便于自动化测试

## 为什么保持 nano 风格

- 不拆多 crate，先单仓单 binary
- 不一次引入过多子系统，先守住最小闭环
- 保留内核边界，避免“业务代码直接调用模型 SDK”

## 扩展约束

- 新功能必须归属于某一层，不允许跨层耦合
- Provider 扩展只能进入 `runtime.rs`
- Kernel 对外只暴露稳定最小接口

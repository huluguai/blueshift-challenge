# Blueshift 挑战集合

本仓库包含 [Blueshift Learning Platform](https://learn.blueshift.gg/zh-CN/challenges) 中的挑战代码实现。

## 项目概述

这个仓库记录了针对 Solana 区块链开发的两个核心挑战，涵盖 SPL 代币编程和 Anchor 框架开发。

---

## 挑战 1: SPL 代币铸造

**文件**: [challenge_1_spl_token.ts](challenge_1_spl_token.ts)

**语言**: TypeScript

**难度**: 初级

### 挑战目标

在单个交易中使用 Web3.js 和 SPL Token 库创建和铸造一个 SPL 代币。

### 主要目标

1. ✅ 创建一个 SPL Mint 账户
2. ✅ 初始化 Mint，设置 6 位小数，费用支付账户为 Mint 权限和冻结权限
3. ✅ 为费用支付账户创建关联代币账户 (ATA)
4. ✅ 铸造 21,000,000 个代币到关联代币账户
5. ✅ 签署并发送交易

### 核心概念

#### Mint 账户创建
- 创建一个新账户，遵守 SPL 代币标准
- 支付租金豁免所需的 SOL，确保账户不被清除
- 指定 TOKEN_PROGRAM_ID 作为账户所有者

#### Mint 初始化
- 设置小数位数为 6（用于计算最小单位）
- 指定 Mint 权限（谁可以铸造代币）
- 指定冻结权限（谁可以冻结代币账户）

#### 关联代币账户 (ATA)
- 使用确定性推导方式计算地址
- 同样的输入总是生成相同的 ATA 地址
- 无需事先在链上创建即可预知地址

#### 代币铸造
- 铸造 21,000,000 个代币（考虑 6 位小数）
- 总供应量为 21,000,000 × 10^6 个最小单位
- 使用 createMintToCheckedInstruction 验证小数位数

### 所需环境变量

- `SECRET`: Base58 编码的密钥对（费用支付账户）
- `RPC_ENDPOINT`: Solana RPC 端点 URL

### 运行方式

```bash
export SECRET=<your_base58_encoded_secret>
export RPC_ENDPOINT=<your_rpc_endpoint>
npx ts-node challenge_1_spl_token.ts
```

---

## 挑战 2: Anchor Vault

**文件**: [challenge_2_anchor_vault.rs](challenge_2_anchor_vault.rs)

**语言**: Rust (Anchor Framework)

**难度**: 中级

### 挑战目标

使用 Anchor 框架创建一个安全的金库程序，支持存取 SOL，使用程序派生地址 (PDA) 进行账户控制。

### 核心功能

#### 1. 存款 (Deposit)

- ✅ 检查金库是否为空（确保每个用户只有一个金库）
- ✅ 验证存款金额超过租金豁免最小值
- ✅ 使用 CPI (跨程序调用) 转账 SOL 到金库账户

**验证条件**:
- 金库初始状态必须为空 (0 lamports)
- 存款金额必须大于租金豁免最小值

#### 2. 取款 (Withdraw)

- ✅ 检查金库是否有 lamports
- ✅ 使用 PDA 签署权限转账所有 SOL
- ✅ 支持全额取款

**关键要点**:
- 使用 `CpiContext::new_with_signer` 进行 PDA 签署
- PDA 的 seeds: `[b"vault", signer_key, bump]`
- 转账使用 System Program 的 Transfer 指令

### 账户结构

```rust
#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,           // 交易签署者
    #[account(mut, seeds = [...], bump)]
    pub vault: SystemAccount<'info>,     // PDA 金库账户
    pub system_program: Program<'info, System>,
}
```

### PDA (程序派生地址)

- **确定性 (Deterministic)**: 相同的 seeds 总是生成相同的地址
- **无私钥**: PDA 本身没有私钥，由程序拥有
- **自动签署**: 在程序上下文中使用 seeds 可以自动签署

**金库地址计算**:
```
seeds = [b"vault", signer.key(), bump]
```

### 错误处理

| 错误 | 触发条件 | 说明 |
|------|--------|------|
| `VaultAlreadyExists` | 金库不为空 | 存款时金库已存在余额 |
| `InvalidAmount` | 金额 ≤ 租金最小值 或 取款时金库为空 | 金额不符合要求 |

### 编译和部署

```bash
# 构建程序
anchor build

# 部署到 devnet
anchor deploy --provider.cluster devnet
```

---

## 技术栈

### 挑战 1 (SPL Token)

- **Web3.js**: Solana JavaScript/TypeScript SDK
- **@solana/spl-token**: SPL Token 库
- **bs58**: Base58 编码/解码

### 挑战 2 (Anchor Vault)

- **Anchor**: Solana 程序框架
- **Rust**: 编程语言
- **Solana Program Library**: 系统程序和工具库

---

## 学习路径

### 初学者

1. 从 `challenge_1_spl_token.ts` 开始理解 SPL 代币的创建和铸造
2. 掌握交易构建、签署和提交的基本流程
3. 理解 Mint 账户、ATA 和小数位数的概念

### 进阶

1. 进行 `challenge_2_anchor_vault.rs` 学习 Anchor 框架
2. 掌握 PDA (程序派生地址) 的概念和应用
3. 理解 CPI (跨程序调用) 和权限验证
4. 实现完整的链上程序逻辑

---

## 相关资源

- 📚 [Blueshift Learning Platform](https://learn.blueshift.gg/zh-CN/challenges)
- 📖 [Solana 官方文档](https://docs.solana.com/)
- 🔗 [Web3.js 文档](https://solana-labs.github.io/solana-web3.js/)
- ⚓ [Anchor 文档](https://www.anchor-lang.com/)
- 🪙 [SPL Token 文档](https://spl.solana.com/token)

---

## 注意事项

⚠️ **安全警告**

- 代码中的密钥用于演示目的，生产环境中**绝不要**在代码中硬编码密钥
- 使用环境变量安全存储敏感信息
- 在主网部署前充分测试（在 devnet 或 testnet 上测试）

---

## 许可证

此项目用于学习和教育目的。

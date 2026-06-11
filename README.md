# Luminous Labs — Contracts

Smart contracts powering Luminous Labs protocols. Audited, battle-tested on-chain logic for real-world asset tokenization, DeFi primitives, and governance.

## Stack

- **Language**: Rust (Soroban SDK) / Solidity
- **Platform**: Stellar (Soroban) + EVM-compatible chains
- **Testing**: Soroban CLI tests / Foundry
- **Deployment**: Hardhat / Soroban deploy scripts
- **Verification**: Etherscan / Stellar Expert

## Structure

```
contracts/
├── core/        — Core protocol logic
├── tokens/      — Fungible & non-fungible tokens
├── governance/  — DAO & voting contracts
├── utils/       — Shared libraries & math
└── test/        — Integration & unit tests
```

## Contributing

See [open issues](https://github.com/Luminous-labs-sec/contracts/issues) for audit findings, feature requests, and optimizations. Please review our security guidelines before submitting PRs.

## License

MIT

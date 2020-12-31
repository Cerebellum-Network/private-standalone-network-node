# How to create Smart Contract artifacts

1. Clone [Cere Enterprise Smart Contracts Repository](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts.git)

2. Run Smart Contract tests
```bash
cargo +nightly test
```

3. Build smart contract from sources
```bash
cargo +nightly contract build
```

4. Generage Contract Metadata
```bash
cargo +nightly contract generate-metadata
```

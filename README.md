# 📦 EtherHiding : Hiding Payloads in plain sight

> **Abstract**: This project demonstrates an experimental system for storing and retrieving executable shellcode from an Ethereum smart contract, and executing it natively on a host system via a secure and minimal Rust client. The goal is to explore alternative payload delivery vectors leveraging decentralized infrastructure and to bridge on-chain storage with off-chain execution contexts.

---

## 🔬 Overview

![POC](https://github.com/user-attachments/assets/1533c387-cf81-42fb-9f3a-231c3ce26658)

This repository contains two components:

- `ShellcodeStorage.sol` — A Solidity smart contract designed to store shellcode in a byte or hex string format on the Ethereum blockchain.
- `main.rs` — A Rust-based executor that queries the deployed smart contract via an Ethereum JSON-RPC provider, extracts the shellcode, allocates executable memory, and executes the payload on the host system.

The system is designed as a **minimal, proof-of-concept payload delivery channel** that merges decentralized storage with native shellcode execution.

---

## 💡 Motivation

As blockchain technologies become more prevalent, they also introduce novel opportunities and attack surfaces. This research investigates:

- **Smart contracts as immutable and censorship-resistant storage** for arbitrary payloads
- **Off-chain interpreters and loaders** written in memory-safe systems programming languages (Rust)
- **Possibility of using Ethereum smart contracts as a C2 (Command & Control)** 

While traditionally shellcode is hosted on centralized C2 infrastructure or bundled into binaries, this project explores a decentralized retrieval model in which the payload cannot be removed once deployed.

---

### 🧱 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- **Windows**
- Address of Already deployed Smart Contract containing calc.exe Shellcode : ``` 0xD57c5867BbD350B97cff9FAd9569089a0Af2EC1d ```

  
---
## ⚙️ Step-by-Step
```powershell
git clone https://github.com/SAAITAAMAA/EtherHiding.git
cd EtherHiding
cargo build
cargo run
```
---
## 📽️ Demo

https://github.com/user-attachments/assets/cc42ca00-5b57-4864-83ac-644a65761800


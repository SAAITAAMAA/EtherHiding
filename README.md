# 📦 EtherHiding : Hiding Payloads in plain sight

> **Abstract**: This project demonstrates an experimental system for storing and retrieving executable shellcode from an Ethereum smart contract, and executing it natively on a host system via a secure and minimal Rust client. The goal is to explore alternative payload delivery vectors leveraging decentralized infrastructure and to bridge on-chain storage with off-chain execution contexts.

---

## 🔬 Overview

![Image](https://github.com/user-attachments/assets/00a1d845-d977-4221-aace-231482cad19e)

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

## 📁 Demo

Address of Already deployed Smart Contract containing calc.exe Shellcode : ``` 0xD57c5867BbD350B97cff9FAd9569089a0Af2EC1d ```

https://github.com/SAAITAAMAA/EtherHiding/issues/1

# 🛡️ Solana Security Reference: Anchor Edition

> **"Security remains the biggest challenge in Solana development. Most exploits aren't high-level math; they are simple, avoidable mistakes."**

Welcome to the **Solana Security Reference**. This repository is a premium educational resource designed to take developers from "clueless" to "security-aware." It contrasts common vulnerable patterns with secure, industry-standard Anchor implementations.

---

## 🗺️ Learning Path

This repository is structured as a progressive curriculum. We recommend following this order:

| Step | Module | Topic | Difficulty | Why it matters |
|:---:|:---|:---|:---:|:---|
| 1 | [01-Ownership](./programs/01-ownership-validation) | **Ownership Validation** | 🟢 Easy | Prevents attackers from spoofing state with fake accounts. |
| 2 | [02-Signer](./programs/02-signer-authorization) | **Signer Authorization** | 🟢 Easy | Ensures only authorized users can trigger sensitive logic. |
| 3 | [03-Data-Matching](./programs/03-account-data-matching) | **Data Matching** | 🟡 Medium | Verifies relationships between accounts (e.g., matching a User to their Profile). |
| 4 | [04-Arithmetic](./programs/04-arithmetic-safety) | **Arithmetic Safety** | 🟡 Medium | Prevents balance manipulation via overflows and underflows. |
| 5 | [05-CPI-Security](./programs/05-cpi-reentrancy) | **CPI & Program IDs** | 🔴 Hard | Guards against malicious cross-program calls and malicious system programs. |

---

## 🧠 The Solana Security Mental Model

To understand Solana security, you must internalize these three pillars:

1. **Don't Trust, Verify**: Every account passed into your instruction is user-controlled. You must verify its Owner, its Signer status, and its relationship to other accounts.
2. **Anchor is Your Shield**: Anchor's `#[derive(Accounts)]` is not just boilerplate—it's your security layer. Use it to declarative security.
3. **Check-Effects-Interactions**: Always validate inputs first, update your internal state second, and call external programs (CPI) last.

---

## 📖 Essential Documentation

- [**BEST_PRACTICES.md**](./SECURITY.md): A concise summary of do's and don'ts.
- [**CHEAT_SHEET.md**](./CHEAT_SHEET.md): A quick-reference guide for Anchor security constraints.
- [**GLOSSARY.md**](./GLOSSARY.md): Definitions for Solana-specific security terms.
- [**RESOURCES.md**](./RESOURCES.md): Where to go next to become a master.

---

## 🛠️ Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/downloads)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)

### Build & Explore
```bash
# Clone the repository
git clone https://github.com/your-repo/solana-security-template
cd solana-security-template

# Build all programs
anchor build
```

---

## 🤝 Contributing & Submitting
This project was built for the **SuperteamNG Security Bounty**. It aims for maximum clarity, readability, and educational impact.

*Built with ❤️ for the Solana Ecosystem.*

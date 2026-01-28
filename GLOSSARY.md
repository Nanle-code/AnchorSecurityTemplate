# 📖 Solana Security Glossary

If you're new to Solana development, these terms are essential for understanding security audits and vulnerabilities.

### Account Ownership
In Solana, every account is owned by a program. Only the owner program can modify the account's data or deduct its balance (lamports).

### Anchor Framework
The most popular framework for building Solana programs. It provides a domain-specific language (DSL) to handle much of the tedious account validation and serialization.

### CPI (Cross-Program Invocation)
When one program calls another program (e.g., your program calling the System Program to transfer SOL). This is a common attack vector if the called program is not carefully validated.

### Lamport
The smallest unit of SOL ($10^{-9}$ SOL). Security vulnerabilities often lead to the draining of lamports from vaults.

### PDA (Program Derived Address)
An account address that is generated deterministically based on a program ID and a set of "seeds." PDAs do not have a private key; they are governed by the program that "signs" for them.

### Re-entrancy
A classic blockchain vulnerability (famous in Ethereum) where a call to another contract (CPI) allows the receiver to call back into the original contract before the state is updated. *Note: Solana's architecture makes true re-entrancy harder than Ethereum, but logic re-entrancy still exists.*

### Serialization / Deserialization
The process of converting raw bytes into structured data (like a Rust `struct`). Security flaws often occur when a program deserializes "fake" data into a trusted struct.

### Signer
An account that has "signed" the transaction with its private key. This proves that the owner of that public key authorized the transaction.

### Underflow / Overflow
Mathematical errors where a number goes below its minimum (wrapping to its maximum) or above its maximum (wrapping to its minimum).

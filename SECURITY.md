# Solana Security Best Practices

This document outlines the core security principles to follow when developing Solana programs with Anchor.

## 1. Always Validate Account Ownership
Never use `UncheckedAccount` or `AccountInfo` unless absolutely necessary (and then, manually check `account.owner == expected_owner`). Use `Account<'info, T>` whenever possible, as Anchor performs this check automatically.

## 2. Check Every Signer
Ensure that any instruction performing an authorized action uses the `Signer<'info>` type. If you are using a custom authority stored in an account, use the `has_one` constraint:
```rust
#[account(has_one = authority)]
pub config: Account<'info, Config>,
pub authority: Signer<'info>,
```

## 3. Use Program Accounts for CPI
When calling other programs, use the `Program<'info, T>` type (e.g., `Program<'info, System>` or `Program<'info, Token>`). This prevents "False Program" attacks where an attacker passes a malicious program ID.

## 4. Use Checked Arithmetic
Always use `.checked_add()`, `.checked_sub()`, etc. While modern Anchor/Rust helps prevent wrapping, explicitly handling overflows with custom errors is safer and more informative.

## 5. Favor PDAs over Keypair Accounts
Program Derived Addresses (PDAs) are more secure because:
- Only your program can sign for them (via `invoke_signed`).
- You can enforce uniqueness by using specific seeds (e.g., a user's pubkey).
- They allow for robust account data matching without manual pointer checks.

## 6. Implement "Check-Effects-Interactions"
1. **Check**: Validate all accounts and signatures (Anchor does most of this in `#[derive(Accounts)]`).
2. **Effects**: Update internal state (deduct balances, update flags).
3. **Interactions**: Perform CPI calls to other programs.

Performing interactions before effects can lead to inconsistent states if the CPI fails or is manipulated.

## 7. Be Careful with `AccountInfo::data`
If you access raw account data, remember that any program can change the data of accounts it owns. Only trust data in accounts owned by *your* program or a trusted system program.

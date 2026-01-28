# Signer Authorization

## The Vulnerability: Missing Signer Check

Instructions that perform sensitive operations (like moving funds or changing settings) must ensure that the authorized party has explicitly signed the transaction.

Failing to check the `is_signer` flag on an account means anyone can provide that account's address and execute the instruction, even if they don't own the private key for that address.

### Vulnerable Code

```rust
pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>, amount: u64) -> Result<()> {
    // ...
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub authority: UncheckedAccount<'info>, // No signature check!
}
```

### The Attack
1. An attacker identifies the `Vault` account and its `authority` (e.g., via a block explorer).
2. The attacker calls `withdraw_vulnerable`, passing the real `authority`'s public key.
3. Since the program doesn't check if `authority` actually signed, the transaction succeeds, and funds are withdrawn to the attacker's destination (or the vault balance is deducted).

## The Fix: Signer Type

Use the `Signer<'info>` type in Anchor. This type wraps `AccountInfo` and enforces a check that `is_signer` is true.

### Secure Code

```rust
#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>, // Anchor ensures this is a signer
}
```

Additional security is provided by `has_one = authority`, which ensures that the `vault.authority` field matches the provided `authority` account.

---

## 🎭 The Pinocchio Perspective

In **Pinocchio**, signature verification is explicit. 

1. You fetch the `account_info` for the authority.
2. You must call `authority.is_signer()` manually.
3. If it returns `false`, you must throw a custom error immediately.

Pinocchio programs are often used for high-performance bots or DEXs where developers might want to conditionally allow non-signer operations (though very rare), whereas Anchor enforces the signer status at the framework level for safety.

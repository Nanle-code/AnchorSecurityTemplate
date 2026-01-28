# Arithmetic Safety

## The Vulnerability: Integer Overflow and Underflow

Integer types have a fixed range (e.g., `u64` max is `18,446,744,073,709,551,615`). Adding to a max value or subtracting from zero can cause "wrapping" or "panics".

- **Wrapping**: `u8::MAX + 1` becomes `0`.
- **Underflow**: `0 - 1` becomes `u8::MAX`.

In Solana, wrapping can lead to critical bugs, such as a user withdrawing more funds than they have because the subtraction wrapped to a huge number.

### Vulnerable Code

```rust
pub fn deposit_vulnerable(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.balance += amount; // Possible overflow
    Ok(())
}
```

### The Attack
If a program uses wrapping arithmetic for balances:
1. An attacker with a balance of `1` "withdraws" `2`.
2. The balance becomes `u64::MAX`.
3. The attacker now has a near-infinite balance.

*Note: Modern Rust and Anchor 0.29+ enable overflow checks by default in `Anchor.toml`, which causes a panic on overflow. However, handling this gracefully with errors is best practice.*

## The Fix: Checked Arithmetic

Always use the `checked_*` methods provided by Rust.

### Secure Code

```rust
pub fn deposit_secure(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    vault.balance = vault.balance.checked_add(amount)
        .ok_or(error!(ErrorCode::Overflow))?;
            
    Ok(())
}
```

This returns a clean error instead of panicking the whole transaction, which is better for both security and user experience.

---

## 🎭 The Pinocchio Perspective

In **Pinocchio**, arithmetic safety is handled exactly like standard Rust. 

1. You use `.checked_add()` or `.checked_sub()`.
2. Since Pinocchio avoids large dependencies, you typically use the core Rust arithmetic methods.
3. Because Pinocchio is often used in performance-critical loops, developers might occasionally use `unchecked_add` if they have *already* validated the range earlier in the program, but this is high-risk and requires extensive testing.

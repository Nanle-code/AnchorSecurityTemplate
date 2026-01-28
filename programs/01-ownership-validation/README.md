# Ownership Validation

## The Vulnerability: Insecure Ownership Check

In Solana, every account has an `owner` field. This is the public key of the program that has the exclusive right to modify the account's data.

When a program receives an account as input, it MUST verify that the account is owned by the expected program. If it doesn't, an attacker can create an account with the exact same data structure, fill it with malicious values, and pass it to the program.

### Vulnerable Code

```rust
pub fn update_admin_vulnerable(ctx: Context<UpdateAdminVulnerableActual>, new_admin: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    // ...
}

#[derive(Accounts)]
pub struct UpdateAdminVulnerableActual<'info> {
    #[account(mut)]
    /// CHECK: This is unsafe because we are not checking the owner!
    pub config: UncheckedAccount<'info>,
    pub admin: Signer<'info>,
}
```

### The Attack
1. An attacker creates their own malicious program.
2. They create a "fake" `Config` account using their program. This account is owned by the *attacker's* program, not the target program.
3. They set the `admin` field in the fake account to their own address.
4. They call `update_admin_vulnerable` passing the fake account.
5. The target program deserializes the account, sees that the `admin` field matches the caller's address, and allows the update.

## The Fix: Account Wrapper

Anchor provides the `Account<'info, T>` type, which automatically performs an ownership check. It ensures that the account's owner is the current program (the one defined in `declare_id!`).

### Secure Code

```rust
#[derive(Accounts)]
pub struct UpdateAdminSecure<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    pub admin: Signer<'info>,
}
```

By using `Account<'info, Config>`, Anchor will:
1. Check that `config.owner == program_id`.
2. Deserialize the data into the `Config` struct.
3. (Optional) perform additional checks like `has_one`.

---

## 🎭 The Pinocchio Perspective

In **Pinocchio**, there are no magic wrappers like `Account`. 

1. You receive a list of raw accounts.
2. You must manually call `account.owner()` and compare it to your Program ID.
3. Because Pinocchio is **zero-copy**, you aren't "deserializing" into a struct; you are reading bytes directly from the memory buffer.

While this is faster (lower Compute Units), it places the burden of security entirely on the developer. If you forget the `owner()` check in Pinocchio, you are 100% vulnerable.

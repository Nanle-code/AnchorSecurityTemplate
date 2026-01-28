# Account Data Matching

## The Vulnerability: Unvalidated Data Relationships

In many programs, one account "belongs" to another (e.g., a `UserProfile` belongs to a `User`). If the program doesn't verify this relationship during an instruction, an attacker can manipulate other users' data.

Even if the attacker is a valid `Signer`, they might be passing a "Profile" account that doesn't belong to them.

### Vulnerable Code

```rust
pub fn update_profile_vulnerable(ctx: Context<UpdateProfileVulnerable>, new_name: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.name = new_name; // Who does this profile belong to?
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateProfileVulnerable<'info> {
    #[account(mut)]
    pub profile: Account<'info, UserProfile>,
    pub user: Signer<'info>, // user is a signer, but is it the RIGHT user?
}
```

### The Attack
1. Alice has a profile. Bob is an attacker.
2. Bob calls `update_profile_vulnerable`.
3. Bob passes **Alice's profile address** but **his own signature**.
4. The program sees that Bob is a signer and Alice's profile is a valid `UserProfile` account.
5. It proceeds to change Alice's name to whatever Bob wants.

## The Fix: `has_one` or PDA Seeds

You must verify that the provided profile contains the user's public key.

### Secure Code

```rust
#[derive(Accounts)]
pub struct UpdateProfileSecure<'info> {
    // has_one = user checks that profile.user == user.key()
    #[account(mut, has_one = user)]
    pub profile: Account<'info, UserProfile>,
    pub user: Signer<'info>,
}
```

Alternatively, using PDAs (Program Derived Addresses) with the user's key as a seed is an even stronger pattern:
```rust
#[account(
    mut,
    seeds = [b"profile", user.key().as_ref()],
    bump
)]
pub profile: Account<'info, UserProfile>,
```
This ensures that there is only one valid profile account for each user.

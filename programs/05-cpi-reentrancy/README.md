# CPI Security: Arbitrary Program ID

## The Vulnerability: Arbitrary CPI Call

Cross-Program Invocation (CPI) allows your program to call other programs (like the System Program or Token Program). When doing so, you must pass the `program_id` of the program you are calling.

If you allow the user to provide this `program_id` without validation, they can pass a malicious program that mimics the expected interface but executes different logic.

### Vulnerable Code

```rust
pub fn transfer_vulnerable(ctx: Context<TransferVulnerable>, amount: u64) -> Result<()> {
    let from = ctx.accounts.from.to_account_info();
    let to = ctx.accounts.to.to_account_info();
    let system_program = ctx.accounts.arbitrary_program.to_account_info();

    // We trust whatever program is passed in arbitrary_program!
    let ix = system_instruction::transfer(from.key, to.key, amount);
    invoke(&ix, &[from, to, system_program])?;
    
    Ok(())
}
```

### The Attack
1. An attacker creates a malicious program that has a "transfer" instruction with the same function signature as the System Program.
2. Inside their malicious transfer, they don't actually move funds, but they log data or perform other unauthorized actions, or they return success despite doing nothing.
3. More dangerously, if the target program *expects* a state change to happen (like a balance deduction), the fake program can spoof this success.

## The Fix: Program Account Type

Anchor provides the `Program<'info, T>` account type. This type automatically verifies that the provided account's public key matches the hardcoded program ID for that type.

### Secure Code

```rust
#[derive(Accounts)]
pub struct TransferSecure<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// SECURE: Anchor checks system_program.key == system_program::ID
    pub system_program: Program<'info, System>,
}
```

By using `Program<'info, System>`, it becomes impossible for a caller to pass any account other than the official Solana System Program.

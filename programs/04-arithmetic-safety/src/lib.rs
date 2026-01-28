use anchor_lang::prelude::*;

declare_id!("L3M4N5O6P7Q8R9S0T1U2V3W4X5Y6Z7A8B9C0D1E2F3G4");

#[program]
pub mod arithmetic_safety {
    use super::*;

    // --- CASE 1: THE VULNERABLE APPROACH ---

    /// VULNERABILITY: Integer Overflow/Underflow
    /// 
    /// This instruction uses standard Rust operators (+, -).
    /// In modern Rust (edition 2021+), these will panic in debug mode
    /// but WRAP in release mode unless specific flags are set.
    /// 
    /// THE EXPLOIT (Wrapping):
    /// If `balance` is 1 and `amount` is u64::MAX, the addition will wrap.
    /// More dangerously, if an attacker "withdraws" 2 from a balance of 1,
    /// a wrapping subtraction would result in a balance of u64::MAX.
    pub fn deposit_vulnerable(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABLE: Direct addition can overflow/wrap.
        vault.balance += amount;
        Ok(())
    }

    // --- CASE 2: THE SECURE (MANUAL CHECK) APPROACH ---

    /// SECURE: Checked Arithmetic
    /// 
    /// This instruction uses the `.checked_add()` method, which returns
    /// an `Option`. If the math would overflow, it returns `None`.
    pub fn deposit_secure(ctx: Context<UpdateBalance>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURE: Explicitly handling the overflow case with a custom error.
        vault.balance = vault.balance.checked_add(amount)
            .ok_or(error!(ErrorCode::Overflow))?;
            
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Vault {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow")]
    Overflow,
}

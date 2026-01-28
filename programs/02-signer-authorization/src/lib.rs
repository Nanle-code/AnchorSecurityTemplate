use anchor_lang::prelude::*;

declare_id!("H5fV7X6M9M9J4x1K7H9W1Z2B3C4D5E6F7G8H9I0J1K2L");

#[program]
pub mod signer_authorization {
    use super::*;

    // --- CASE 1: THE VULNERABLE APPROACH ---

    /// VULNERABILITY: Missing Signer Verification
    /// 
    /// This instruction allows anyone to "withdraw" from the vault.
    /// It asks for an `authority` account, but never checks if that authority
    /// actually signed the transaction.
    /// 
    /// THE EXPLOIT:
    /// An attacker can pass the real authority's public key as the `authority` account.
    /// Because the program doesn't check `authority.is_signer`, it will proceed.
    pub fn withdraw_vulnerable(ctx: Context<WithdrawVulnerable>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABLE: We are trusting the `authority.key` without verifying the signature.
        if vault.authority != ctx.accounts.authority.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        vault.balance -= amount;
        msg!("Withdrew {} from vault (VULNERABLE)", amount);
        Ok(())
    }

    // --- CASE 2: THE SECURE (ANCHOR) APPROACH ---

    /// SECURE: Explicit Signer Type
    /// 
    /// This instruction uses Anchor's `Signer` type. 
    /// If the `authority` account did not sign the transaction, 
    /// the transaction will fail before it even reaches this logic.
    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURE: Anchor has verified that `authority` is a signer.
        // SECURE: The `has_one` constraint in the struct ensures `vault.authority == authority`.
        
        vault.balance -= amount;
        msg!("Withdrew {} from vault (SECURE)", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawVulnerable<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// VULNERABLE: `AccountInfo` (or UncheckedAccount) does not verify signatures.
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    /// SECURE: Using the `Signer` type forces Anchor to check `is_signer`.
    pub authority: Signer<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

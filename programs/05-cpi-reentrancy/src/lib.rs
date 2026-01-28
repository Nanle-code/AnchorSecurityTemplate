use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

declare_id!("N5O6P7Q8R9S0T1U2V3W4X5Y6Z7A8B9C0D1E2F3G4H5I6");

#[program]
pub mod cpi_security {
    use super::*;

    // --- CASE 1: THE VULNERABLE APPROACH ---

    /// VULNERABILITY: Arbitrary Program ID Injection
    /// 
    /// This instruction is intended to transfer SOL using the System Program.
    /// However, it accepts the `system_program` as a raw `AccountInfo` 
    /// without verifying that it IS the real System Program.
    /// 
    /// THE EXPLOIT:
    /// An attacker can create a malicious program that mimics the System Program's 
    /// "transfer" interface. They pass THIS malicious program ID.
    /// Our program will then call the attacker's program, giving it control 
    /// over the instruction flow and potentially leaking sensitive data or 
    /// spoofing a successful transfer.
    pub fn transfer_vulnerable(ctx: Context<TransferVulnerable>, amount: u64) -> Result<()> {
        let from = ctx.accounts.from.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let system_program = ctx.accounts.arbitrary_program.to_account_info();

        // VULNERABLE: We are invoking a program that could be anything!
        let ix = system_instruction::transfer(from.key, to.key, amount);
        invoke(&ix, &[from, to, system_program])?;
        
        Ok(())
    }

    // --- CASE 2: THE SECURE (ANCHOR) APPROACH ---

    /// SECURE: Explicit Program Type
    /// 
    /// This instruction uses Anchor's `Program<'info, System>` type.
    /// Anchor will automatically verify that the account provided matches 
    /// the official Solana System Program ID.
    pub fn transfer_secure(ctx: Context<TransferSecure>, amount: u64) -> Result<()> {
        let from = ctx.accounts.from.to_account_info();
        let to = ctx.accounts.to.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // SECURE: Anchor has verified that system_program is indeed the System Program.
        let ix = system_instruction::transfer(from.key, to.key, amount);
        invoke(&ix, &[from, to, system_program])?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferVulnerable<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// VULNERABLE: No check on this program ID.
    pub arbitrary_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferSecure<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// SECURE: Anchor checks system_program.key == system_program::ID
    pub system_program: Program<'info, System>,
}

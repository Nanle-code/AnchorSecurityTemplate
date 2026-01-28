use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ownership_validation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = admin;
        Ok(())
    }

    // --- CASE 1: THE VULNERABLE APPROACH ---
    
    /// VULNERABILITY: Insecure Ownership Verification
    /// 
    /// This instruction is intended to allow the admin to update their own address.
    /// However, it fails to verify who OWNS the `config` account.
    /// 
    /// WHY IS THIS DANGEROUS?
    /// An attacker can create a "Fake Config" account in a different program they control.
    /// They can populate that fake account with data that matches our `Config` struct 
    /// (e.g., setting the `admin` field to their own public key).
    /// When they pass this fake account to our program, our program will trust it.
    pub fn update_admin_vulnerable(ctx: Context<UpdateAdminVulnerableActual>, new_admin: Pubkey) -> Result<()> {
        let config_info = &ctx.accounts.config;
        
        // At this point, we haven't checked if this account was actually created by OUR program.
        // We are just blindly deserializing bytes.
        let mut data = config_info.try_borrow_mut_data()?;
        let mut config = Config::try_deserialize(&mut &data[..])?;

        if config.admin != ctx.accounts.admin.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        config.admin = new_admin;
        
        // Manually saving back... this is also error-prone!
        let mut writer = &mut data[..];
        config.try_serialize(&mut writer)?;

        Ok(())
    }

    // --- CASE 2: THE SECURE (ANCHOR) APPROACH ---

    /// SECURE: Built-in Ownership Check
    /// 
    /// This instruction uses Anchor's `Account` type. 
    /// Anchor handles the "heavy lifting" of security for you.
    pub fn update_admin_secure(ctx: Context<UpdateAdminSecure>, new_admin: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        // Anchor has already verified:
        // 1. config.to_account_info().owner == program_id
        // 2. config.admin == ctx.accounts.admin.key() (via the 'has_one' constraint)
        // 3. admin.is_signer == true (via the 'Signer' type)

        config.admin = new_admin;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdminVulnerableActual<'info> {
    #[account(mut)]
    /// CHECK: This is a raw AccountInfo. There are NO checks on its owner or data.
    pub config: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdminSecure<'info> {
    /// SECURE: The `Account` wrapper checks that the account is owned by THIS program.
    /// SECURE: The `has_one` constraint ensures the internal `admin` field matches the `admin` account provided.
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    
    /// SECURE: The `Signer` type ensures the user actually signed this transaction.
    pub admin: Signer<'info>,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Admin signature required")]
    Unauthorized,
}

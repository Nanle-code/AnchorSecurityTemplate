use anchor_lang::prelude::*;

declare_id!("J1K2L3M4N5O6P7Q8R9S0T1U2V3W4X5Y6Z7A8B9C0D1E2");

#[program]
pub mod data_matching {
    use super::*;

    // --- CASE 1: THE VULNERABLE APPROACH ---

    /// VULNERABILITY: Unvalidated Data Relationships
    /// 
    /// This instruction allows a user to update a profile.
    /// It verifies that the caller is a `Signer`, but it never checks
    /// if the `profile` account actually belongs to that `Signer`.
    /// 
    /// THE EXPLOIT:
    /// Alice has a profile. Bob is an attacker.
    /// Bob calls this instruction with HIS signature, but passes ALICE'S profile address.
    /// The program sees Bob is a signer, and blindly updates Alice's profile.
    pub fn update_profile_vulnerable(ctx: Context<UpdateProfileVulnerable>, new_name: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        
        // VULNERABLE: No check that profile.user == user.key()
        profile.name = new_name;
        Ok(())
    }

    // --- CASE 2: THE SECURE (ANCHOR) APPROACH ---

    /// SECURE: Constraint-based Validation
    /// 
    /// This instruction uses the `has_one` constraint to strictly link 
    /// the profile with the signer.
    pub fn update_profile_secure(ctx: Context<UpdateProfileSecure>, new_name: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        
        // SECURE: Anchor has verified that profile.user == ctx.accounts.user.key()
        profile.name = new_name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfileVulnerable<'info> {
    #[account(mut)]
    pub profile: Account<'info, UserProfile>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateProfileSecure<'info> {
    /// SECURE: `has_one = user` checks that the `user` field in the `UserProfile` struct
    /// matches the public key of the `user` account provided in the instruction.
    #[account(mut, has_one = user)]
    pub profile: Account<'info, UserProfile>,
    pub user: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub user: Pubkey,
    pub name: String,
}

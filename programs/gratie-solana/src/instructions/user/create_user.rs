use anchor_lang::{prelude::*, solana_program::{system_instruction, program::invoke, native_token::LAMPORTS_PER_SOL}};
use crate::{state::{user::User, company_license::CompanyLicense, tier::Tier}, error::MyError};

pub fn create_user_handler(ctx: Context<CreateUser>, user_id: String, encrypted_private_key: String, user_password_encryption_algorithm: u8, user_password_salt: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    
    if !ctx.accounts.company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    // Calculate user limit 
    let free_user_limit = ctx.accounts.tier.free_user_limit as u64;
    let user_limit = free_user_limit + ctx.accounts.company_license.paid_user_limit;

    // Check if user limit is reached
    if ctx.accounts.company_license.user_count >= user_limit {
        return Err(MyError::MaxUsersReached.into());
    }

    if encrypted_private_key.len() > 300 {
        return Err(MyError::EncryptedPrivateKeyTooLong.into());
    }

    // Increment user count
    ctx.accounts.company_license.user_count += 1;

    user.owner = ctx.accounts.user_account.key();
    user.company = ctx.accounts.mint_authority.key();
    user.user_id = user_id;
    user.encrypted_private_key = Some(encrypted_private_key);
    user.user_password_encryption_algorithm = Some(user_password_encryption_algorithm);
    user.user_password_salt = Some(user_password_salt);
    user.bump = *ctx.bumps.get("user").ok_or(MyError::BumpNotFound)?;

    let transfer = system_instruction::transfer(
        &ctx.accounts.mint_authority.key(),
        // The lamports are sent to the gratie_wallet that lives on the program
        &ctx.accounts.user_account.key(), 
        10_000_000,
    );

    invoke(&transfer, &[
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.user_account.to_account_info(),
    ])?;


    Ok(())
}

#[derive(Accounts)]
#[instruction(
    company_name: String,
    user_id: String,
    encrypted_private_key: String,
    user_password_encryption_algorithm: u8,
    user_password_encryption_salt: String
)]
pub struct CreateUser<'info> {
    #[account(mut, address = company_license.owner)]
    pub mint_authority: Signer<'info>,
    
    #[account(mut, seeds = [b"company_license".as_ref(), company_name.as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(address = company_license.tier)]
    pub tier: Account<'info, Tier>,


    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    
    #[account(
        init, 
        payer = mint_authority,
        space = User::LEN,
        seeds = [b"user".as_ref(), company_license.key().as_ref(), user_id.as_bytes()], 
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

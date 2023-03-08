use anchor_lang::prelude::*;
use crate::{state::{user::User, company_license::CompanyLicense}, error::MyError};

pub fn create_user_handler(ctx: Context<CreateUser>, user_id: String, encrypted_private_key: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    
    if !ctx.accounts.company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    user.owner = ctx.accounts.user_account.key();
    user.company = ctx.accounts.mint_authority.key();
    user.user_id = user_id;
    user.encrypted_private_key = Some(encrypted_private_key);
    user.bump = *ctx.bumps.get("user").ok_or(MyError::BumpNotFound)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    user_id: String,
    encrypted_private_key: String
)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    
    #[account(mut, seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,


    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    
    #[account(
        init, 
        payer = mint_authority,
        space = User::LEN,
        seeds = [b"user".as_ref(), company_license.key().as_ref(), user_account.key().as_ref()], 
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

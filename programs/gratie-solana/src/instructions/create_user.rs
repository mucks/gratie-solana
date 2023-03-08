use anchor_lang::prelude::*;
use crate::{state::{user::User, company_license::CompanyLicense}, error::MyError};

pub fn create_user_handler(ctx: Context<CreateUser>, user_id: String, encrypted_private_key: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    
    if !ctx.accounts.company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    user.company = ctx.accounts.mint_authority.key();
    user.owner = ctx.accounts.user_account.key();
    user.user_id = user_id;
    user.encrypted_private_key = Some(encrypted_private_key);

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
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    // verify the company license
    #[account(mut, seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,
    
    #[account(
        init, 
        payer = mint_authority,
        space = User::LEN,
        // maybe use user email here as seed
        seeds = [b"user".as_ref(), company_license.key().as_ref(), user_account.key().as_ref()], 
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

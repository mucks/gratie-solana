use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::state::{user::User, user_rewards_bucket::{UserRewardsBucket, self}, company_license::CompanyLicense, company_rewards_bucket::CompanyRewardsBucket};

use anchor_spl::associated_token;

pub fn create_user_rewards_bucket_token_account_handler(
    ctx: Context<CreateUserRewardsBucketTokenAccount>,
) -> Result<()> {
    ctx.accounts.user_rewards_bucket.token_account = Some(ctx.accounts.token_account.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(company_name: String, user_id: String)]
pub struct CreateUserRewardsBucketTokenAccount<'info> {
    #[account(mut, address = company_license.owner)]
    pub mint_authority: Signer<'info>,

    #[account(
        mut, 
        seeds = [b"company_license".as_ref(), company_name.as_ref()],
        bump = company_license.bump
    )]
    pub company_license: Account<'info, CompanyLicense>,
    
    #[account(mut, seeds = [b"company_rewards_bucket".as_ref(), company_license.key().as_ref()], bump = company_rewards_bucket.bump)]
    pub company_rewards_bucket: Account<'info, CompanyRewardsBucket>,
    
    #[account(mut, address = company_rewards_bucket.token_mint_key)]
    pub mint: Account<'info, token::Mint>,

    #[account(mut, seeds = [b"user_rewards_bucket".as_ref(), user.key().as_ref()], bump=user_rewards_bucket.bump)]
    pub user_rewards_bucket: Account<'info, UserRewardsBucket>,

    #[account(mut, 
        seeds = [b"user".as_ref(), company_license.key().as_ref(), user_id.as_bytes()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = user.owner)]
    pub user_account: AccountInfo<'info>,

    #[account(
        init,
        // only have one token account per bucket
        // user_reward_bucket_token_account
        payer = mint_authority, 
        // the mint address of the tokens the company minted
        associated_token::mint = mint,
        // user_rewards_bucket itself owns the tokens 
        associated_token::authority =  user_account,
    )]
    pub token_account: Account<'info, token::TokenAccount>,

    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    pub system_program: Program<'info, System>
}

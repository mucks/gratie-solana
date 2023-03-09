use anchor_lang::prelude::*;
use crate::{state::{user_rewards_bucket::UserRewardsBucket, company_license::{CompanyLicense}, user::{User}, company_rewards_bucket::CompanyRewardsBucket, tier::Tier}, error::MyError};

// a user can currently only have one bucket maximum

// ERC-1155
pub fn create_user_rewards_bucket_handler(ctx: Context<CreateUserRewardsBucket>) -> Result<()> {
    let user_rewards_bucket = &mut ctx.accounts.user_rewards_bucket;

    user_rewards_bucket.user = ctx.accounts.user.key();
    user_rewards_bucket.creator = ctx.accounts.mint_authority.key();
    user_rewards_bucket.token_account = ctx.accounts.token_account.key();
    user_rewards_bucket.created_at = Clock::get()?.unix_timestamp;

    ctx.accounts.company_rewards_bucket.user_rewards_bucket_count += 1;

    user_rewards_bucket.bump = *ctx.bumps.get("user_rewards_bucket").ok_or(MyError::BumpNotFound)?;

    Ok(())
}
// Create user reward buckets from a common merkle root

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateUserRewardsBucket<'info> {
    #[account(mut, address = company_license.owner)]
    pub mint_authority: Signer<'info>,
    
    #[account(mut)]
    pub user: Account<'info, User>,

    #[account(
        mut, 
        seeds = [b"company_license".as_ref(), company_name.as_ref()],
        bump = company_license.bump
    )]
    pub company_license: Account<'info, CompanyLicense>,


    #[account(mut, seeds = [b"company_rewards_bucket".as_ref(), company_license.key().as_ref()], bump = company_rewards_bucket.bump)]
    pub company_rewards_bucket: Account<'info, CompanyRewardsBucket>,

    // unsure about the mint_authority_key not sure if user can then access it
    // however we need some company identifier
    // right now the seed is the static text "user_reward_bucket" + mint_authority_key + user_account_key
    // so the user needs some way to get that company publickey somewhere
    // otherwise we need to store it in the user account
    // or we add a method that lets the user with that key get the company pubkey
    #[account(
        init, 
        payer = mint_authority,
        space = UserRewardsBucket::LEN,
        seeds = [b"user_rewards_bucket".as_ref(), user.key().as_ref()], 
        bump
    )]
    pub user_rewards_bucket: Account<'info, UserRewardsBucket>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
}

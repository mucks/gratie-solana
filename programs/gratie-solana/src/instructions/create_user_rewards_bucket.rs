use anchor_lang::prelude::*;
use crate::{state::{user_rewards_bucket::UserRewardsBucket, company_license::{CompanyLicense, self}, user::User}, error::MyError};


// ERC-1155
pub fn create_user_rewards_bucket_handler(ctx: Context<CreateUserRewardsBucket>) -> Result<()> {
    let user_rewards_bucket = &mut ctx.accounts.user_rewards_bucket;


    if !ctx.accounts.company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    if ctx.accounts.company_license.rewards_token_account.is_none() {
        return Err(MyError::CompanyLicenseHasNotMintedRewards.into());
    }

    user_rewards_bucket.user = ctx.accounts.user.key();
    user_rewards_bucket.creator = ctx.accounts.mint_authority.key();
    user_rewards_bucket.token_account = ctx.accounts.token_account.key();

    ctx.accounts.company_license.user_rewards_bucket_count += 1;

    Ok(())
}
// Create user reward buckets from a common merkle root

#[derive(Accounts)]
pub struct CreateUserRewardsBucket<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    
    #[account(mut)]
    pub user: Account<'info, User>,


    // verify the company license
    #[account(mut, seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,
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

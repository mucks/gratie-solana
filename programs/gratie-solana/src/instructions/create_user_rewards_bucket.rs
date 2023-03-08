use anchor_lang::prelude::*;
use crate::{state::{user_rewards_bucket::UserRewardsBucket, company_license::{CompanyLicense, self}}, error::MyError};

pub fn create_user_rewards_bucket_handler(ctx: Context<CreateUserRewardsBucket>, user_id: String, encrypted_private_key: String) -> Result<()> {
    let user_rewards_bucket = &mut ctx.accounts.user_rewards_bucket;


    if !ctx.accounts.company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }
    if ctx.accounts.company_license.rewards_token_account.is_none() {
        return Err(MyError::CompanyLicenseHasNotMintedRewards.into());
    }

    user_rewards_bucket.user = ctx.accounts.user_account.key();
    user_rewards_bucket.creator = ctx.accounts.mint_authority.key();
    user_rewards_bucket.token_account = ctx.accounts.token_account.key();
    user_rewards_bucket.encrypted_private_key = Some(encrypted_private_key);
    user_rewards_bucket.user_id = user_id;

    ctx.accounts.company_license.user_rewards_bucket_count += 1;

    Ok(())
}
// Create user reward buckets from a common merkle root

#[derive(Accounts)]
#[instruction(
    user_email: String,
    encrypted_private_key: String
)]
pub struct CreateUserRewardsBucket<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_account: AccountInfo<'info>,


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
        // maybe use user email here as seed
        seeds = [b"user_rewards_bucket".as_ref(), mint_authority.key().as_ref(), user_account.key().as_ref()], 
        bump
    )]
    pub user_rewards_bucket: Account<'info, UserRewardsBucket>,
    pub system_program: Program<'info, System>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
}

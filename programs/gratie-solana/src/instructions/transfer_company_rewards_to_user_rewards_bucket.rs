use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};

use crate::{
    error::MyError,
    state::{
        company_license::CompanyLicense, company_rewards_bucket::CompanyRewardsBucket, user::User,
        user_rewards_bucket::UserRewardsBucket,
    },
};

pub fn transfer_company_rewards_to_user_rewards_bucket_handler(
    ctx: Context<TransferCompanyRewardsToUserRewardsBucket>,
    amount: u64,
) -> Result<()> {
    // Check if the input token accounts are the same as the ones in the state
    if &ctx.accounts.from.token_account != ctx.accounts.from_token_account.key
        || &ctx.accounts.to.token_account != ctx.accounts.to_token_account.key
    {
        return Err(MyError::InvalidTokenAccount.into());
    }

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.sender.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    //TODO: verify that the amount is smaller than the balance of the company_rewards_bucket

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(company_name: String, user_id: String, amount: u64)]
pub struct TransferCompanyRewardsToUserRewardsBucket<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(seeds = [b"company_license".as_ref(), company_name.as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(mut, seeds = [b"company_rewards_bucket".as_ref(), company_license.key().as_ref()], bump = from.bump)]
    pub from: Account<'info, CompanyRewardsBucket>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from_token_account: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"user_rewards_bucket", user.key().as_ref()], bump = to.bump)]
    pub to: Account<'info, UserRewardsBucket>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to_token_account: UncheckedAccount<'info>,

    #[account(seeds = [b"user".as_ref(), company_license.key().as_ref(), user_id.as_ref()], bump = user.bump)]
    pub user: Account<'info, User>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

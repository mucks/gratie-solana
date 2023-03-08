use crate::error::MyError;
use crate::state::company_license::CompanyLicense;
use crate::state::company_rewards_bucket::CompanyRewardsBucket;
use anchor_lang::prelude::*;
use anchor_spl::token;

pub fn create_company_rewards_bucket_handler(
    ctx: Context<CreateCompanyRewardsBucket>,
) -> Result<()> {
    let company_license = &mut ctx.accounts.company_license;

    // Check if the company license is verified
    if !company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    // Create the tokens
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::mint_to(cpi_ctx, company_license.evaluation)?;

    let company_rewards_bucket = &mut ctx.accounts.company_rewards_bucket;
    company_rewards_bucket.creator = ctx.accounts.mint_authority.key();
    company_rewards_bucket.company_license = company_license.key();
    company_rewards_bucket.token_account = ctx.accounts.token_account.key();
    company_rewards_bucket.token_mint_key = ctx.accounts.mint.key();
    company_rewards_bucket.user_rewards_bucket_count = 0;
    company_rewards_bucket.created_at = Clock::get()?.unix_timestamp;
    
    // TODO: figure out what the bump does exactly
    company_rewards_bucket.bump = *ctx.bumps.get("company_rewards_bucket").ok_or(MyError::BumpNotFound)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCompanyRewardsBucket<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut, seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(
        init, 
        payer = mint_authority,
        space = CompanyRewardsBucket::LEN,
        // seed ensures that company_license can only have one bucket
        seeds = [b"company_rewards_bucket".as_ref(), company_license.key().as_ref()], 
        bump
    )]
    pub company_rewards_bucket: Account<'info, CompanyRewardsBucket>,
    
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    // The token program is the program that will be used to mint the token.
    pub token_program: Program<'info, token::Token>,
}
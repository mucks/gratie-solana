use crate::error::MyError;
use crate::state::company_license::CompanyLicense;
use anchor_lang::prelude::*;
use anchor_spl::token;

pub fn create_company_rewards_handler(ctx: Context<CreateCompanyRewards>) -> Result<()> {
    let company_license = &mut ctx.accounts.company_license;

    // Check if the company license is verified
    if !company_license.verified {
        return Err(MyError::CompanyLicenseNotVerified.into());
    }

    // Check if the company license has already minted rewards
    if company_license.has_minted_rewards {
        return Err(MyError::CompanyLicenseAlreadyMintedRewards.into());
    }

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::mint_to(cpi_ctx, company_license.evaluation)?;

    company_license.has_minted_rewards = true;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCompanyRewards<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut, seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()], bump = company_license.bump)]
    pub company_license: Account<'info, CompanyLicense>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    // The token program is the program that will be used to mint the token.
    pub token_program: Program<'info, token::Token>,
}

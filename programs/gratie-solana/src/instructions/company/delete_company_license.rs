use crate::state::company_license::CompanyLicense;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Burn, Mint, TokenAccount};

// Deletes the company license account and the company license token
pub fn delete_company_license_handler(ctx: Context<DeleteCompanyLicense>) -> Result<()> {
    // let cpi_accounts = Burn {
    //     mint: ctx.accounts.mint.to_account_info(),
    //     from: ctx.accounts.company_license_owner.to_account_info(),
    //     authority: ctx.accounts.company_license_owner.to_account_info(),
    // };
    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // token::burn(cpi_ctx, 1)?;

    Ok(())
}

#[derive(Accounts)]
pub struct DeleteCompanyLicense<'info> {
    #[account(mut, address = company_license.owner)]
    pub company_license_owner: Signer<'info>,
    #[account(mut, close = company_license_owner)]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(mut, address = company_license.token_account)]
    pub company_license_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = company_license.mint)]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, token::Token>,
}

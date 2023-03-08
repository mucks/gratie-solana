use anchor_lang::prelude::*;
use crate::state::company_license::CompanyLicense;
use crate::error::MyError;
use anchor_spl::token;

// This creates a unique company license for each wallet address.
// Note: the name here is not unique
// TODO: find a way to make the name unique
// This transaction will fail if the name is too long or the wallet address already has a company license.
// This license can already be queried in the frontend

pub fn create_company_license_handler(ctx: Context<CreateCompanyLicense>, name: String, email: String, logo_uri: String, evaluation: u64, tier: u8) -> Result<()> {
    if ctx.accounts.company_license.token_account.is_some() {
        return Err(MyError::CompanyLicenseAlreadyExists.into());
    }
    if name.as_bytes().len() > 200 {
        return Err(MyError::NameTooLong.into());
    }
    if email.as_bytes().len() > 200 {
        return Err(MyError::EmailTooLong.into());
    }
    if logo_uri.as_bytes().len() > 200 {
        return Err(MyError::UriTooLong.into());
    }

    let company_license = &mut ctx.accounts.company_license;

    company_license.name = name;
    company_license.email = email;
    company_license.logo_uri = logo_uri;
    company_license.evaluation = evaluation;
    company_license.tier = tier;

    // Create the token
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Create a single token for the company license
    token::mint_to(cpi_ctx, 1)?;

    company_license.token_account = Some(ctx.accounts.token_account.key());


    // TODO: figure out what the bump does exactly
    company_license.bump = *ctx.bumps.get("company_license").ok_or(MyError::BumpNotFound)?;
    Ok(())

}


#[derive(Accounts)]
#[instruction(
    name: String,
    email: String,
    logo_uri: String,
    evaluation: u64,
    tier: u8
)]
pub struct CreateCompanyLicense<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(
        init, 
        payer = mint_authority,
        space = CompanyLicense::LEN,
        // This seed ensures that only one wallet can have one token
        seeds = [b"company_license".as_ref(), mint_authority.key().as_ref()],
        bump
    )]
    pub company_license: Account<'info, CompanyLicense>,
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


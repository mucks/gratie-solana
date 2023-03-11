use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use crate::state::company_license::CompanyLicense;
use crate::error::MyError;
use crate::state::gratie_wallet::GratieWallet;
use crate::state::tier::Tier;
use anchor_spl::token;

// This creates a unique company license for each wallet address.
// Note: the name here is not unique
// TODO: find a way to make the name unique
// This transaction will fail if the name is too long or the wallet address already has a company license.
// This license can already be queried in the frontend

pub fn create_company_license_handler(ctx: Context<CreateCompanyLicense>, name: String, email: String, token_metadata_json_uri: String, evaluation: u64) -> Result<()> {
    if name.as_bytes().len() > 200 {
        return Err(MyError::NameTooLong.into());
    }
    if email.as_bytes().len() > 200 {
        return Err(MyError::EmailTooLong.into());
    }
    if token_metadata_json_uri.as_bytes().len() > 200 {
        return Err(MyError::UriTooLong.into());
    }

    // Company pays the fee for creation

    let transfer = system_instruction::transfer(
        &ctx.accounts.mint_authority.key(),
        // The lamports are sent to the gratie_wallet that lives on the program
        &ctx.accounts.gratie_wallet.key(), 
        ctx.accounts.tier.price_lamports
    );

    invoke(&transfer, &[
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.gratie_wallet.to_account_info(),
    ])?;

    ctx.accounts.gratie_wallet.amount_earned += ctx.accounts.tier.price_lamports as u128;

    // Company license is created

    let company_license = &mut ctx.accounts.company_license;

    company_license.name = name;
    company_license.email = email;
    company_license.token_metadata_json_uri = token_metadata_json_uri;
    company_license.evaluation = evaluation;
    company_license.tier = ctx.accounts.tier.key();
    company_license.owner = ctx.accounts.mint_authority.key();

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

    company_license.token_account = ctx.accounts.token_account.key();
    company_license.mint = ctx.accounts.mint.key();


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
    tier_id: u8
)]
pub struct CreateCompanyLicense<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(
        init, 
        payer = mint_authority,
        space = CompanyLicense::LEN,
        // This seed ensures that only one wallet can have one token
        seeds = [b"company_license".as_ref(), name.as_ref()],
        bump
    )]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(mut, seeds = [b"gratie_wallet".as_ref()], bump = gratie_wallet.bump)]
    pub gratie_wallet: Account<'info, GratieWallet>,

    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    
    // The token program is the program that will be used to mint the token.
    pub token_program: Program<'info, token::Token>,


    #[account(mut, seeds = [b"tier".as_ref(), &[tier_id]], bump)]
    pub tier: Account<'info, Tier>,

}


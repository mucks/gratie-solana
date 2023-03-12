// doesn't work on localnet because of metaplex
// this error is thrown: Error processing Instruction 0: instruction expected an executable account

use crate::state::company_license::CompanyLicense;
use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::token::{self, Mint, TokenAccount};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

pub fn add_company_license_to_metaplex_handler(
    ctx: Context<AddCompanyLicenseToMetaplexContext>,
) -> Result<()> {
    // Metadata
    let metadata_accounts = vec![
        // Metadata account
        ctx.accounts.metadata.to_account_info(),
        // Mint
        ctx.accounts.mint.to_account_info(),
        // Mint Authority, the company that is minting the license.
        ctx.accounts.company_license_owner.to_account_info(),
        // Payer, the company that is minting the license.
        ctx.accounts.company_license_owner.to_account_info(),
        // The program that is used to create the metadata accounts.
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    let symbol = "GRATIE".to_string();

    let create_metadata_accounts = create_metadata_accounts_v3(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.company_license_owner.key(),
        // title
        ctx.accounts.company_license.name.clone(),
        symbol,
        ctx.accounts.company_license.token_metadata_json_uri.clone(),
        None,
        1,
        true,
        false,
        None,
        None,
        None,
    );

    invoke(&create_metadata_accounts, &metadata_accounts)?;

    msg!("Metadata Accounts Created !!!");

    // Master Edition
    let master_edition_accounts = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.company_license_owner.to_account_info(),
        ctx.accounts.company_license_owner.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    let create_master_edition = create_master_edition_v3(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.master_edition.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.company_license_owner.key(),
        Some(0),
    );

    invoke(&create_master_edition, &master_edition_accounts)?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddCompanyLicenseToMetaplexContext<'info> {
    #[account(mut, address = company_license.owner)]
    pub company_license_owner: Signer<'info>,

    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,

    #[account(mut, address = company_license.token_account)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut, address = company_license.mint)]
    pub mint: Account<'info, Mint>,

    // Related to metaplex
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    // Related to metaplex
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    // The token program is the program that will be used to mint the token.
    pub token_program: Program<'info, token::Token>,
    // Create new accounts, allocate account data, assign accounts to owning programs, transfer lamports from System Program owned accounts and pay transaction fees.
    pub system_program: Program<'info, System>,

    // ? The program that is used to create the metadata accounts.
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
}

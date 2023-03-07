use anchor_lang::prelude::*;

use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

/*
    Will be minted by a company when they want to use our service.
    This will be a unique token for each company.
    One wallet address can only have one company license.
*/

// mint company license using metaplex
// TODO: implement a check to see if the wallet address already has a company license.
// TODO: use pdas to create a unique hash with the program id and the users public key

pub fn mint_company_license_to_metaplex_handler(
    ctx: Context<MintCompanyLicenseMetaplex>,
    creator_key: Pubkey,
    uri: String,
    title: String,
) -> Result<()> {
    msg!("Minting Company License");

    // Create a new account for the token.
    // CPI stands for cross program invocation.
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    msg!("CPI Accounts Assigned");

    // Get the CPI Program
    let cpi_program = ctx.accounts.token_program.to_account_info();
    msg!("CPI Program Assigned");

    // Create a new CPI Context
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    msg!("CPI Context Assigned");

    // Mint token to the token account.
    token::mint_to(cpi_ctx, 1)?;
    msg!("Token Minted !!!");

    // Create the metadata accounts.

    // Get the account info
    let account_info = vec![
        // Metadata account
        ctx.accounts.metadata.to_account_info(),
        // Mint
        ctx.accounts.mint.to_account_info(),
        // Mint Authority, the company that is minting the license.
        ctx.accounts.mint_authority.to_account_info(),
        // Payer, the company that is minting the license.
        ctx.accounts.payer.to_account_info(),
        // The program that is used to create the metadata accounts.
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    msg!("Account Info Assigned");

    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: creator_key,
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 0,
        },
    ];

    let symbol: String = "GRATIE".to_string();

    invoke(
        &create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            title,
            symbol,
            uri,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;
    msg!("Metadata Accounts Created !!!");

    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;

    msg!("Master Edition Nft Minted !!!");

    Ok(())
}

// Metaplex related stuff
#[derive(Accounts)]
pub struct MintCompanyLicenseMetaplex<'info> {
    // ? The account of the company that is minting the license.
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    // ? The account of the company that is minting the license.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

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

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::state::TokenMetadataAccount;

// The Id of our program.
declare_id!("6Ki5zkofcL2h3q1F1T9WYUPWwbu9DogCGB5vmLGyi1Ce");

// Why is rust showing an error here but compiling successfully?
#[program]
pub mod gratie_solana {

    use mpl_token_metadata::state::Collection;

    use super::*;

    pub fn get_metadata(ctx: Context<GetMetadata>) -> Result<Pubkey> {
        let (metadata, _) = Pubkey::find_program_address(
            &[
                mpl_token_metadata::state::PREFIX.as_bytes(),
                mpl_token_metadata::id().as_ref(),
                ctx.accounts.mint_authority.key.as_ref(),
            ],
            &mpl_token_metadata::id(),
        );
        Ok(metadata)
    }

    // Input wallet address
    pub fn get_company_license(ctx: Context<GetCompanyLicense>) -> Result<Option<Collection>> {
        msg!("Getting Company License");

        let mint_metadata = Metadata::from_account_info(&ctx.accounts.metadata.to_account_info())?;
        msg!("Mint Metadata Assigned");

        Ok(mint_metadata.collection)
    }

    // This creates a unique company license for each wallet address.
    // Note: the name here is not unique
    // TODO: somehow connect this to the metaplex nft
    pub fn create_company_license(ctx: Context<CreateCompanyLicense>, name: String) -> Result<()> {
        let company_license = &mut ctx.accounts.company_license;
        if name.as_bytes().len() > 200 {
            return Err(MyError::NameTooLong.into());
        }
        company_license.name = name;
        company_license.bump = *ctx.bumps.get("company_license").unwrap();
        Ok(())
    }
    
    /*
        Will be minted by a company when they want to use our service.
        This will be a unique token for each company.
        One wallet address can only have one company license.
    */

    // mint company license using metaplex
    // TODO: implement a check to see if the wallet address already has a company license.
    // TODO: use pdas to create a unique hash with the program id and the users public key
    pub fn mint_company_license_metaplex(
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
}

#[error_code]
pub enum MyError {
    #[msg("Name needs to be less than 200 characters")]
    NameTooLong,
}

#[derive(Accounts)]
pub struct GetMetadata<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCompanyLicense<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}

#[account]
pub struct CompanyLicense {
    tier: u8,
    name: String,
    bump: u8,
}

// Company License related stuff
#[derive(Accounts)]
pub struct CreateCompanyLicense<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 1 tier + 4 name length + 200 name + 1 bump
    #[account(
        init, 
        payer = user,
        space = 8 + 1 + 4 + 200 + 1,
        seeds = [b"company_license".as_ref(), user.key().as_ref()],
        bump
    )]
    pub company_license: Account<'info, CompanyLicense>,
    pub system_program: Program<'info, System>,
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

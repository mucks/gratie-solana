// doesn't work on localnet because of metaplex
// this error is thrown: Error processing Instruction 0: instruction expected an executable account

use crate::state::{company_license::CompanyLicense, company_rewards_bucket::CompanyRewardsBucket};
use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::token;
use mpl_token_metadata::instruction::create_metadata_accounts_v3;

pub fn add_company_reward_tokens_to_metaplex_handler(
    ctx: Context<AddCompanyRewardTokensToMetaplexContext>,
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

    let create_metadata_accounts = create_metadata_accounts_v3(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.company_license_owner.key(),
        ctx.accounts.company_license_owner.key(),
        // title
        ctx.accounts.company_rewards_bucket.token_name.clone(),
        ctx.accounts.company_rewards_bucket.token_symbol.clone(),
        ctx.accounts
            .company_rewards_bucket
            .token_metadata_json_uri
            .clone(),
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

    Ok(())
}

#[derive(Accounts)]
pub struct AddCompanyRewardTokensToMetaplexContext<'info> {
    #[account(mut, address = company_license.owner)]
    pub company_license_owner: Signer<'info>,

    #[account(mut, seeds = [b"company_rewards_bucket".as_ref(), company_license.key().as_ref()], bump = company_rewards_bucket.bump)]
    pub company_rewards_bucket: Account<'info, CompanyRewardsBucket>,

    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = company_rewards_bucket.token_account)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, address = company_rewards_bucket.token_mint_key)]
    pub mint: UncheckedAccount<'info>,

    // Related to metaplex
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    // The token program is the program that will be used to mint the token.
    pub token_program: Program<'info, token::Token>,
    // Create new accounts, allocate account data, assign accounts to owning programs, transfer lamports from System Program owned accounts and pay transaction fees.
    pub system_program: Program<'info, System>,

    // ? The program that is used to create the metadata accounts.
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
}

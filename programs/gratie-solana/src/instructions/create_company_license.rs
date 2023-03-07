use anchor_lang::prelude::*;
use crate::state::company_license::CompanyLicense;
use crate::error::MyError;

// This creates a unique company license for each wallet address.
// Note: the name here is not unique
// TODO: somehow connect this to the metaplex nft
// TODO: find a way to make the name unique
// This transaction will fail if the name is too long or the wallet address already has a company license.
// This license can already be queried in the frontend

pub fn create_company_license_handler(ctx: Context<CreateCompanyLicense>, name: String, email: String, logo_uri: String, evaluation: u64, tier: u8) -> Result<()> {
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



    // TODO: figure out what the bump does exactly
    company_license.bump = *ctx.bumps.get("company_license").unwrap();
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
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user,
        space = CompanyLicense::LEN,
        seeds = [b"company_license".as_ref(), user.key().as_ref()],
        bump
    )]
    pub company_license: Account<'info, CompanyLicense>,
    pub system_program: Program<'info, System>,
}


use anchor_lang::prelude::*;
use crate::metaplex::*;
    
mod metaplex;
mod error;
mod admin;

// NOTE: program id changes when me change the account structure

// The Id of our program.
declare_id!("AEf99S19YTaox9E8aX3ugpGJtHTHjaQtSY2ixtLysFGr");

// If there is an index len error make sure that every method has a context

// Why is rust showing an error here but compiling successfully?
#[program]
pub mod gratie_solana {

    use crate::admin::is_admin;

    use super::*;

    // This creates a unique company license for each wallet address.
    // Note: the name here is not unique
    // TODO: somehow connect this to the metaplex nft
    // TODO: find a way to make the name unique
    // This transaction will fail if the name is too long or the wallet address already has a company license.
    // This license can already be queried in the frontend
    pub fn create_company_license(ctx: Context<CreateCompanyLicense>, name: String, email: String, logo_uri: String, evaluation: u32, tier: u8) -> Result<()> {
        // String size validation
        // TODO: make this more efficient and readable
        if name.as_bytes().len() > 200 {
            return Err(error::MyError::NameTooLong.into());
        }
        if email.as_bytes().len() > 200 {
            return Err(error::MyError::EmailTooLong.into());
        }
        if logo_uri.as_bytes().len() > 200 {
            return Err(error::MyError::UriTooLong.into());
        }

        let company_license = &mut ctx.accounts.company_license;


        company_license.name = name;
        company_license.email = email;
        company_license.logo_uri = logo_uri;
        company_license.evalutation = evaluation;
        company_license.tier = tier;



        // TODO: figure out what the bump does exactly
        company_license.bump = *ctx.bumps.get("company_license").unwrap();
        Ok(())
    }

    // TODO: only me as the owner of the program can verify a company license
    pub fn verify_company_license(ctx: Context<VerifyCompanyLicense>) -> Result<()> {
        if !is_admin(ctx.accounts.admin.key) {
            return Err(error::MyError::NotAdmin.into());
        }

        let company_license = &mut ctx.accounts.company_license;
        company_license.verified = true;
        Ok(())
    }



    
    pub fn mint_company_license_metaplex(
        ctx: Context<MintCompanyLicenseMetaplex>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        mint_company_license_to_metaplex_handler(ctx, creator_key, uri, title)?;
        Ok(())
    }
}

// #[account]
// pub struct CompanyLicenseRegister {
//     pub address: Pubkey,
//     pub counter: u32,
// }


// TODO: connect this account to the nft metadata account
#[account]
#[derive(Default)]
pub struct CompanyLicense {
    // the tier of the company (small, midsized, large, etc)
    tier: u8,

    name: String,
    email: String,
    // images should be saved on arweave and the url should be stored here
    // company_logo_uri: String,
    logo_uri: String,

    // the evaluation of the company
    evalutation: u32,
    // add a method that only lets us change the verified status if the company is verified
    // add approveCompany function that only me the admin can sign 
    // admin is an approved whitelisted wallet that we own
    verified: bool,
    // TODO: figure out what the bump does exactly
    bump: u8,
}

impl CompanyLicense {
    // + 600 is for 3 strings
    pub const LEN: usize = std::mem::size_of::<Self>() + 600;
}

#[derive(Accounts)]
pub struct VerifyCompanyLicense<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,
}

#[derive(Accounts)]
#[instruction(
    name: String,
    email: String,
    logo_uri: String,
    evaluation: u32,
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

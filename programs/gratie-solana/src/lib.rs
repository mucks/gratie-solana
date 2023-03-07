use crate::instructions::*;
use crate::metaplex::*;
use anchor_lang::prelude::*;

mod admin;
mod error;
mod instructions;
mod metaplex;
mod state;

// NOTE: program id changes when me change the account structure

// The Id of our program.
declare_id!("AEf99S19YTaox9E8aX3ugpGJtHTHjaQtSY2ixtLysFGr");

// If there is an index len error make sure that every method has a context

// Why is rust showing an error here but compiling successfully?
#[program]
pub mod gratie_solana {
    use super::*;

    pub fn create_company_license(
        ctx: Context<CreateCompanyLicense>,
        name: String,
        email: String,
        logo_uri: String,
        evaluation: u64,
        tier: u8,
    ) -> Result<()> {
        create_company_license_handler(ctx, name, email, logo_uri, evaluation, tier)
    }

    pub fn create_company_rewards(ctx: Context<CreateCompanyRewards>) -> Result<()> {
        create_company_rewards_handler(ctx)
    }

    pub fn verify_company_license(ctx: Context<VerifyCompanyLicense>) -> Result<()> {
        verify_company_license_handler(ctx)
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

    // pub fn exit(ctx: Context<Ctor>) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct ExitContext {}

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

    pub fn withdraw_from_gratie_wallet(
        ctx: Context<WithdrawFromGratieWallet>,
        amount_lamports: u64,
    ) -> Result<()> {
        withdraw_from_gratie_wallet_handler(ctx, amount_lamports)
    }

    pub fn create_gratie_wallet(ctx: Context<CreateGratieWallet>) -> Result<()> {
        create_gratie_wallet_handler(ctx)
    }

    // ERC-721
    pub fn create_company_license(
        ctx: Context<CreateCompanyLicense>,
        name: String,
        email: String,
        logo_uri: String,
        evaluation: u64,
        tier_id: u8,
    ) -> Result<()> {
        create_company_license_handler(ctx, name, email, logo_uri, evaluation, tier_id)
    }

    // ERC-1155
    pub fn create_user_rewards_bucket(
        ctx: Context<CreateUserRewardsBucket>,
        company_name: String,
        user_id: String,
    ) -> Result<()> {
        create_user_rewards_bucket_handler(ctx)
    }

    // ERC-20
    pub fn create_company_rewards_bucket(
        ctx: Context<CreateCompanyRewardsBucket>,
        company_name: String,
    ) -> Result<()> {
        create_company_rewards_bucket_handler(ctx)
    }

    pub fn transfer_company_rewards_to_user_rewards_bucket(
        ctx: Context<TransferCompanyRewardsToUserRewardsBucket>,
        company_name: String,
        user_id: String,
        amount: u64,
    ) -> Result<()> {
        transfer_company_rewards_to_user_rewards_bucket_handler(ctx, amount)
    }

    pub fn claim_user(
        ctx: Context<ClaimUser>,
        new_user_public_key: Pubkey,
        new_user_encrypted_private_key: String,
    ) -> Result<()> {
        claim_user_handler(ctx, new_user_public_key, new_user_encrypted_private_key)
    }

    pub fn claim_user_to_his_own_wallet(
        ctx: Context<ClaimUserToHisOwnWallet>,
        new_user_public_key: Pubkey,
    ) -> Result<()> {
        claim_user_to_his_own_wallet_handler(ctx, new_user_public_key)
    }

    pub fn create_user(
        ctx: Context<CreateUser>,
        company_name: String,
        user_id: String,
        encrypted_private_key: String,
        user_password_encryption_algorithm: u8,
        user_password_salt: String,
    ) -> Result<()> {
        create_user_handler(
            ctx,
            user_id,
            encrypted_private_key,
            user_password_encryption_algorithm,
            user_password_salt,
        )
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

    pub fn create_tier(
        ctx: Context<CreateTier>,
        id: u8,
        name: String,
        free_user_limit: u32,
        price_lamports: u64,
        additional_user_price_lamports: u64,
        platform_fee_permille: u16,
    ) -> Result<()> {
        create_tier_handler(
            ctx,
            id,
            name,
            free_user_limit,
            price_lamports,
            additional_user_price_lamports,
            platform_fee_permille,
        )
    }

    // pub fn exit(ctx: Context<Ctor>) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct ExitContext {}

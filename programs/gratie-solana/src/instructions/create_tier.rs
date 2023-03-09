use crate::admin::admin_pubkey;
use crate::{error::MyError, state::tier::Tier};
use anchor_lang::prelude::*;

pub fn create_tier_handler(
    ctx: Context<CreateTier>,
    id: u8,
    name: String,
    free_user_limit: u32,
    price_lamports: u64,
    additional_user_price_lamports: u64,
    platform_fee_permille: u16,
) -> Result<()> {
    let tier = &mut ctx.accounts.tier;
    tier.creator = *ctx.accounts.creator.key;
    tier.id = id;
    tier.name = name;
    tier.free_user_limit = free_user_limit;
    tier.price_lamports = price_lamports;
    tier.additional_user_price_lamports = additional_user_price_lamports;
    tier.platform_fee_permille = platform_fee_permille;
    tier.bump = *ctx.bumps.get("tier").ok_or(MyError::BumpNotFound)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u8, name: String, free_user_limit: u32, price_lamports: u64, additional_user_price_lamports: u64, platform_fee_permille: u16)]
pub struct CreateTier<'info> {
    #[account(mut, address = admin_pubkey())]
    pub creator: Signer<'info>,
    #[account(init, payer = creator, space = Tier::LEN, seeds = [b"tier".as_ref(), &[id]], bump)]
    pub tier: Account<'info, Tier>,
    pub system_program: Program<'info, System>,
}

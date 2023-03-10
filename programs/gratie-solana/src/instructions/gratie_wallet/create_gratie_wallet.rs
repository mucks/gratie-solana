use crate::{error::MyError, instructions::is_admin_handler, state::gratie_wallet::GratieWallet};
use anchor_lang::prelude::*;

pub fn create_gratie_wallet_handler(ctx: Context<CreateGratieWallet>) -> Result<()> {
    is_admin_handler(ctx.accounts.owner.key)?;

    let gratie_wallet = &mut ctx.accounts.gratie_wallet;
    gratie_wallet.owner = *ctx.accounts.owner.key;
    gratie_wallet.bump = *ctx
        .bumps
        .get("gratie_wallet")
        .ok_or(MyError::BumpNotFound)?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateGratieWallet<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = GratieWallet::LEN, seeds = [b"gratie_wallet".as_ref()], bump)]
    pub gratie_wallet: Account<'info, GratieWallet>,
    pub system_program: Program<'info, System>,
}

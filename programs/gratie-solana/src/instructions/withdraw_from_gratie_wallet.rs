// only admin should be able to withdraw from the gratie_wallet

use anchor_lang::prelude::*;

use crate::{
    admin::admin_pubkey,
    error::MyError,
    state::gratie_wallet::{self, GratieWallet},
};

pub fn withdraw_from_gratie_wallet_handler(
    ctx: Context<WithdrawFromGratieWallet>,
    amount_lamports: u64,
) -> Result<()> {
    let gratie_wallet = &mut ctx.accounts.gratie_wallet;
    let withdrawer = &ctx.accounts.withdrawer;

    let rent_balance = Rent::get()?.minimum_balance(GratieWallet::LEN);

    if **gratie_wallet.to_account_info().lamports.borrow() < amount_lamports + rent_balance {
        return Err(MyError::InsufficientFunds.into());
    }

    // Takes the amount from the gratie_wallet and adds it to the withdrawer
    **gratie_wallet.to_account_info().try_borrow_mut_lamports()? -= amount_lamports;
    **withdrawer.to_account_info().try_borrow_mut_lamports()? += amount_lamports;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFromGratieWallet<'info> {
    // later all admins are allowed to withdraw
    // there should be limits on how much can be withdrawn by each admin and by day
    #[account(mut, address = admin_pubkey())]
    pub withdrawer: Signer<'info>,
    #[account(mut, seeds = [b"gratie_wallet".as_ref()], bump = gratie_wallet.bump)]
    pub gratie_wallet: Account<'info, GratieWallet>,
    pub system_program: Program<'info, System>,
}

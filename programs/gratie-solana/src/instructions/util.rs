use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token;

pub fn transfer_all_lamports<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
) -> Result<()> {
    let rent_balance = Rent::get()?.minimum_balance(token::TokenAccount::LEN);
    let rest_balance_lamports = **from.to_account_info().lamports.borrow() - rent_balance;

    let transfer = system_instruction::transfer(&from.key(), &to.key(), rest_balance_lamports);

    invoke(&transfer, &[from, to])?;

    Ok(())
}

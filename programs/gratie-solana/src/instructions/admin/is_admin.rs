use std::str::FromStr;

use crate::error::MyError;
use anchor_lang::prelude::*;

pub fn is_admin_handler(admin_pubkey: &Pubkey) -> Result<()> {
    if !is_admin(admin_pubkey) {
        return Err(MyError::NotAdmin.into());
    }
    Ok(())
}

#[derive(Accounts)]
pub struct CheckIsAdminContext {}

fn admins() -> Vec<Pubkey> {
    vec![
        Pubkey::from_str("3BS9v3KU5TBgavqr9Ws2hmKLs4cD8MjdvvMUrcBTQayw").unwrap(),
        Pubkey::from_str("CHkVNFD1gSrF6pruwF4FxJ3JuPymTZEzuKvFau3tEGkC").unwrap(),
        Pubkey::from_str("FYYQrD437f5mxszLKtutCWd3EKdHmpbuayudBdWwbu8W").unwrap(),
    ]
}

fn is_admin(pubkey: &Pubkey) -> bool {
    admins().contains(pubkey)
}

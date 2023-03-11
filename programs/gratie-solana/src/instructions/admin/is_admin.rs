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
        // mucks
        Pubkey::from_str("EeuPm1L85Vcxh5o3CA3ESCAT1r58EjYvsFWUvhWYDAu7").unwrap(),
        // selva
        Pubkey::from_str("CHkVNFD1gSrF6pruwF4FxJ3JuPymTZEzuKvFau3tEGkC").unwrap(),
        // ragul
        Pubkey::from_str("FYYQrD437f5mxszLKtutCWd3EKdHmpbuayudBdWwbu8W").unwrap(),
    ]
}

fn is_admin(pubkey: &Pubkey) -> bool {
    admins().contains(pubkey)
}

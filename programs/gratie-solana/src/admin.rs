use std::str::FromStr;

use anchor_lang::prelude::Pubkey;

// TODO: could define a super admin that can then add more admins through the contract
// this is currently my pubkey
// pub const ADMIN_PUBKEY: Pubkey =
//     Pubkey::new_from_array(include_bytes!("../config/admin_public_key.txt"));
// Pubkey::from_str("3BS9v3KU5TBgavqr9Ws2hmKLs4cD8MjdvvMUrcBTQayw").unwrap();

pub fn admin_pubkey() -> Pubkey {
    Pubkey::from_str("3BS9v3KU5TBgavqr9Ws2hmKLs4cD8MjdvvMUrcBTQayw").unwrap()
}

pub fn is_admin(pubkey: &Pubkey) -> bool {
    pubkey == &admin_pubkey()
}

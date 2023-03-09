use std::str::FromStr;

use anchor_lang::prelude::Pubkey;

pub fn admin_pubkey() -> Pubkey {
    Pubkey::from_str("3BS9v3KU5TBgavqr9Ws2hmKLs4cD8MjdvvMUrcBTQayw").unwrap()
}

pub fn is_admin(pubkey: &Pubkey) -> bool {
    pubkey == &admin_pubkey()
}

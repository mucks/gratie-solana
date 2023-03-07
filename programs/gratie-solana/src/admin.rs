use anchor_lang::prelude::Pubkey;

// TODO: could define a super admin that can then add more admins through the contract
// this is currently my pubkey
const ADMIN_PUBKEY: &str = "3BS9v3KU5TBgavqr9Ws2hmKLs4cD8MjdvvMUrcBTQayw";

pub fn is_admin(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == ADMIN_PUBKEY
}

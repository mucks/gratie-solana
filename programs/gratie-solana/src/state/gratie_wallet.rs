// This is our wallet that all the fees are send to

use anchor_lang::prelude::*;

#[account]
pub struct GratieWallet {
    pub owner: Pubkey,
    // Amount of fees that have been collected in lamports
    pub amount_earned: u128,
    pub bump: u8,
}

impl GratieWallet {
    pub const LEN: usize = std::mem::size_of::<Self>() + 50;
}

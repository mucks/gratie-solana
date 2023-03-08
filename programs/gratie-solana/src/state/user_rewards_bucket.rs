use anchor_lang::prelude::*;

// ERC-1155
#[account]
#[derive(Default)]
pub struct UserRewardsBucket {
    // company who created the bucket
    pub creator: Pubkey,
    // bucket owner
    pub user: Pubkey,

    // where the rewards are stored
    pub token_account: Pubkey,
    // timestamp when the bucket was created
    // should be created when the user signs up
    // Note: we can add a lot of timebased rewards with this
    pub created_at: i64,

    // timestamp when the bucket was claimed by the user
    pub claimed_at: Option<i64>,
    pub bump: u8,
}

impl UserRewardsBucket {
    pub const LEN: usize = std::mem::size_of::<Self>() + 400;
}

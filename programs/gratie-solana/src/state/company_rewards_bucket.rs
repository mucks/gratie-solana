use anchor_lang::prelude::*;

// contains the ERC-20 tokens
#[account]
#[derive(Default)]
pub struct CompanyRewardsBucket {
    // company who created the bucket
    pub creator: Pubkey,
    pub company_license: Pubkey,

    // holder of the erc-20 tokens
    pub token_account: Pubkey,
    // mint of the erc-20 tokens
    pub token_mint_key: Pubkey,

    pub user_rewards_bucket_count: u64,

    // timestamp when the bucket was created
    pub created_at: i64,

    pub bump: u8,
}

impl CompanyRewardsBucket {
    pub const LEN: usize = std::mem::size_of::<Self>() + 200;
}

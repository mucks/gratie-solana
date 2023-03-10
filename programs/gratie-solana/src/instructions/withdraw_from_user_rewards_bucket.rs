use anchor_lang::prelude::*;

use crate::state::{user::User, user_rewards_bucket::UserRewardsBucket};

pub fn withdraw_from_user_rewards_bucket_handler(
    ctx: Context<WithdrawFromUserRewardsBucket>,
    amount: u64,
) {
}

#[derive(Accounts)]
pub struct WithdrawFromUserRewardsBucket<'info> {
    #[account(mut, address = user.owner)]
    pub withdrawer: Signer<'info>,
    #[account(mut, seeds = [b"user_rewards_bucket", user.key().as_ref()], bump = user_rewards_bucket.bump)]
    pub user_rewards_bucket: Account<'info, UserRewardsBucket>,
    #[account(mut)]
    pub user: Account<'info, User>,
}

// Create user reward buckets from a common merkle root

#[derive(Accounts)]
pub struct CreateUserRewardBucket<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    // unsure about the mint_authority_key not sure if user can then access it
    // however we need some company identifier
    // right now the seed is the static text "user_reward_bucket" + mint_authority_key + user_account_key
    // so the user needs some way to get that company publickey somewhere
    // otherwise we need to store it in the user account
    // or we add a method that lets the user with that key get the company pubkey
    #[account(init, payer=mint_authority, seeds = [b"user_reward_bucket".as_ref(), mint_authority.key().as_ref(), user_account.key()], bump)]
    pub user_reward_bucket: Account<'info, UserRewardBucket>,
}

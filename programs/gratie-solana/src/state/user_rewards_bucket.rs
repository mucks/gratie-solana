use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserRewardsBucket {
    // company who created the bucket
    pub creator: Pubkey,
    pub user: Pubkey,

    pub user_email: String,

    // the company (in frontend) creates a private key that's encrypted with the company_name, user_email and user_password (encrypted) and some other secret
    // this keypair should be derived from the company license pubkey
    // the problem here is that the company will have full access to that private key
    // so when the user claims his bucket a new private key should be encrypted with that users password
    // this way the user can decrypt his private key in the frontend and use it to sign transactions
    // this option is by default Some() but can be set to None if the user moves the bucket to his own wallet
    // the user should be incentivized to move the bucket to his own wallet
    // WARN: this is probably very unsafe!
    pub encrypted_private_key: Option<String>,

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

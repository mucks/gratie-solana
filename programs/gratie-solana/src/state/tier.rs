use anchor_lang::prelude::*;

// This is the license tier that a company can choose from
// we (gratie) have to create these tiers
// we need a way to limit our power to change these tiers
// because if we can change them all the time that removes trust from the users

#[account]
pub struct Tier {
    pub creator: Pubkey,
    pub id: u8,
    pub name: String,
    pub free_user_limit: u32,
    pub price_lamports: u64,
    pub additional_user_price_lamports: u64,
    // 1/1000 can be changed to allow more precision
    pub platform_fee_permille: u16,

    pub company_license_count: u32,

    pub bump: u8,
}
impl Tier {
    pub const LEN: usize = std::mem::size_of::<Self>() + 200;
}

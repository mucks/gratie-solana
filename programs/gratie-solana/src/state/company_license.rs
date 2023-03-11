use anchor_lang::prelude::*;

// TODO: connect this account to the nft metadata account
// The CompanyLicense is the Account that owns everything that the company has access to
#[account]
#[derive(Default)]
pub struct CompanyLicense {
    // owner of the company_license nft
    pub owner: Pubkey,

    pub tier: Pubkey,

    pub name: String,
    pub email: String,

    // the evaluation of the company
    pub evaluation: u64,
    // add a method that only lets us change the verified status if the company is verified
    // add approveCompany function that only me the admin can sign
    // admin is an approved whitelisted wallet that we own
    pub verified: bool,

    // the token account of the company license
    pub token_account: Pubkey,

    // metadata json for the company license token (for metaplex)
    pub token_metadata_json_uri: String,

    pub mint: Pubkey,

    pub user_count: u64,

    // The company can buy additional user slots
    pub paid_user_limit: u64,

    // TODO: figure out what the bump does exactly
    pub bump: u8,
}

impl CompanyLicense {
    // + 600 is for 3 strings
    pub const LEN: usize = std::mem::size_of::<Self>() + 600;
}

use anchor_lang::prelude::*;

// TODO: connect this account to the nft metadata account
#[account]
#[derive(Default)]
pub struct CompanyLicense {
    // the tier of the company (small, midsized, large, etc)
    pub tier: u8,

    pub name: String,
    pub email: String,
    // images should be saved on arweave and the url should be stored here
    // company_logo_uri: String,
    pub logo_uri: String,

    // the evaluation of the company
    pub evalutation: u32,
    // add a method that only lets us change the verified status if the company is verified
    // add approveCompany function that only me the admin can sign
    // admin is an approved whitelisted wallet that we own
    pub verified: bool,
    // TODO: figure out what the bump does exactly
    pub bump: u8,
}

impl CompanyLicense {
    // + 600 is for 3 strings
    pub const LEN: usize = std::mem::size_of::<Self>() + 600;
}

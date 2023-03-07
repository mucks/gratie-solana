use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Name needs to be less than 200 characters")]
    NameTooLong,
    #[msg("Email needs to be less than 200 characters")]
    EmailTooLong,
    #[msg("Uri needs to be less than 200 characters")]
    UriTooLong,
    #[msg("Unauthorized, You are not an admin")]
    NotAdmin,
}

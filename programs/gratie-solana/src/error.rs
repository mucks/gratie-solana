use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Name needs to be less than 200 characters")]
    NameTooLong,
    #[msg("Email needs to be less than 200 characters")]
    EmailTooLong,
    #[msg("Uri needs to be less than 200 characters")]
    UriTooLong,
    #[msg("Your company license already exists!")]
    CompanyLicenseAlreadyExists,
    #[msg("Your company license is not verified yet!")]
    CompanyLicenseNotVerified,
    #[msg("Your company license has already minted rewards!")]
    CompanyLicenseAlreadyMintedRewards,
    #[msg("Your company license has not minted any rewards yet!")]
    CompanyLicenseHasNotMintedRewards,
    #[msg("Unauthorized, You are not an admin")]
    NotAdmin,
    #[msg("Bump was not found on object in context")]
    BumpNotFound,
    #[msg("max users reached")]
    MaxUsersReached,
    #[msg("Account has insufficient funds")]
    InsufficientFunds,
    #[msg("Encrypted Private Key is too long")]
    EncryptedPrivateKeyTooLong,
    #[msg("Invalid Token Account")]
    InvalidTokenAccount,
    #[msg("Invalid Old Token Account")]
    InvalidOldTokenAccount,
    #[msg("Invalid New Account")]
    InvalidNewTokenAccount,
    #[msg("Invalid Tier")]
    InvalidTier,
    #[msg("User has already been claimed")]
    UserAlreadyClaimed,
    #[msg("User has not been claimed yet, user needs to be claimed in order to claim to his own wallet")]
    UserNotClaimedYet,
    #[msg("User has already been claimed to his own wallet")]
    UserAlreadyClaimedToHisOwnWallet,
}

use crate::error::MyError;
use crate::{admin::is_admin, state::company_license::CompanyLicense};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyCompanyLicense<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,
}

pub fn verify_company_license_handler(ctx: Context<VerifyCompanyLicense>) -> Result<()> {
    if !is_admin(ctx.accounts.admin.key) {
        return Err(MyError::NotAdmin.into());
    }

    let company_license = &mut ctx.accounts.company_license;
    company_license.verified = true;
    Ok(())
}

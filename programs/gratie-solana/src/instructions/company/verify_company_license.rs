use crate::{
    error::MyError, instructions::is_admin_handler, state::company_license::CompanyLicense,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyCompanyLicense<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,
}

pub fn verify_company_license_handler(ctx: Context<VerifyCompanyLicense>) -> Result<()> {
    is_admin_handler(ctx.accounts.admin.key)?;
    let company_license = &mut ctx.accounts.company_license;
    company_license.verified = true;
    Ok(())
}

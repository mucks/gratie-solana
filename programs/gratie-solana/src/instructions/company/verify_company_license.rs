use crate::admin::admin_pubkey;
use crate::state::company_license::CompanyLicense;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyCompanyLicense<'info> {
    #[account(mut, address = admin_pubkey())]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub company_license: Account<'info, CompanyLicense>,
}

pub fn verify_company_license_handler(ctx: Context<VerifyCompanyLicense>) -> Result<()> {
    let company_license = &mut ctx.accounts.company_license;
    company_license.verified = true;
    Ok(())
}

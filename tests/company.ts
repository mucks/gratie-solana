import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import { GratieSolana } from "../target/types/gratie_solana";
import { COMPANY_NAME } from "./gratie-solana";
import { getCompanyLicense, getCompanyLicensePDA, getCompanyRewardsBucketPDA, getGratieWalletPDA, getTierPDA } from "./pda";
import { createMintKeyAndTokenAccount } from "./util";


export const createCompanyRewardsBucket = async (program: anchor.Program<GratieSolana>, wallet: anchor.Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);
  const tokenName = "Example Company Token";
  const tokenSymbol = "ECT";
  const tokenMetadataJsonUrl = "https://raw.githubusercontent.com/mucks/gratie-solana/master/assets/company-reward-tokens-sample.json";

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet.publicKey);


  await program.methods.createCompanyRewardsBucket(COMPANY_NAME, tokenName, tokenSymbol, tokenMetadataJsonUrl).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    companyRewardsBucket: companyRewardsBucketPDA,
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).rpc();

}


export const verifyCompanyLicense = async (program: anchor.Program<GratieSolana>, wallet: anchor.Wallet) => {
  const companyLicense = getCompanyLicensePDA(program, COMPANY_NAME);
  await program.methods.verifyCompanyLicense().accounts({ admin: wallet.publicKey, companyLicense: companyLicense }).rpc();


  const updatedLicense = await getCompanyLicense(program, COMPANY_NAME);
  expect(updatedLicense.verified).to.equal(true);
}

export const createCompanyLicense = async (program: anchor.Program<GratieSolana>, wallet: anchor.Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);

  const testEmail = "mail@mucks.dev";
  const testEvaluation = new anchor.BN(100000);
  const tierID = 1;
  const tierPDA = getTierPDA(program, tierID);
  const tier = await program.account.tier.fetch(tierPDA);
  const tokenMetadataJsonUrl = "https://raw.githubusercontent.com/mucks/gratie-solana/master/assets/company-license-sample.json";

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet.publicKey);

  const gratieWalletBefore = await program.account.gratieWallet.fetch(getGratieWalletPDA(program));
  const oldAmountEarned = gratieWalletBefore.amountEarned.toNumber();


  await program.methods.createCompanyLicense(COMPANY_NAME, testEmail, tokenMetadataJsonUrl, testEvaluation, tierID).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    gratieWallet: getGratieWalletPDA(program),
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    tier: tierPDA,
  }).rpc();

  const companyLicense = await getCompanyLicense(program, COMPANY_NAME);
  console.log(companyLicense);
  expect(companyLicense.name).to.equal(COMPANY_NAME);

  // check if the amountEarned increased by the price of the tier
  const gratieWallet = await program.account.gratieWallet.fetch(getGratieWalletPDA(program));
  const amountEarnedDiff = gratieWallet.amountEarned.toNumber() - oldAmountEarned;
  expect(amountEarnedDiff).to.equal(tier.priceLamports.toNumber());
}


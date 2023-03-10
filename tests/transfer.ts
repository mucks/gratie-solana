import * as anchor from "@project-serum/anchor";
import { getCompanyLicense, getCompanyLicensePDA, getCompanyRewardsBucketPDA, getGratieWalletPDA, getTierPDA, getUserPDA, getUserRewardsBucketPDA } from "./pda";

import { GratieSolana } from "../target/types/gratie_solana";
import { COMPANY_NAME, USER_ID } from "./gratie-solana";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";


export const transferTokensToUser = async (program: anchor.Program<GratieSolana>, wallet: anchor.Wallet, amount: anchor.BN) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);
  const companyRewardsBucket = await program.account.companyRewardsBucket.fetch(companyRewardsBucketPDA);

  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);
  const user = await program.account.user.fetch(userPDA);
  const userRewardsBucketPDA = getUserRewardsBucketPDA(program, userPDA);

  const userRewardsBucket = await program.account.userRewardsBucket.fetch(userRewardsBucketPDA);


  await program.methods.transferCompanyRewardsToUserRewardsBucket(COMPANY_NAME, USER_ID, amount).accounts({
    sender: wallet.publicKey,
    companyLicense: companyLicensePDA,
    from: companyRewardsBucketPDA,
    fromTokenAccount: companyRewardsBucket.tokenAccount,
    to: userRewardsBucketPDA,
    toTokenAccount: userRewardsBucket.tokenAccount,
    user: userPDA,
    tokenProgram: TOKEN_PROGRAM_ID,
    userAccount: user.owner,
  }).rpc();

}
import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";

export const getUserPDA = async (program: Program<GratieSolana>, companyLicensePublicKey: anchor.web3.PublicKey, userAccountPublicKey: anchor.web3.PublicKey) => {
  const [userPDA, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('user'),
      companyLicensePublicKey.toBuffer(),
      userAccountPublicKey.toBuffer(),
    ],
    program.programId
  );
  return userPDA;
}

export const getCompanyLicensePDA = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const [companyLicensePDA, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('company_license'),
      wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  return companyLicensePDA;
}

export const getCompanyRewardsBucketPDA = async (program: Program<GratieSolana>, companyLicensePDA: anchor.web3.PublicKey) => {
  const [companyRewardsBucket, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('company_rewards_bucket'),
      companyLicensePDA.toBuffer(),
    ],
    program.programId
  );
  return companyRewardsBucket;
}

export const getCompanyRewardsBucket = async (program: Program<GratieSolana>, companyLicensePDA: anchor.web3.PublicKey) => {
  const companyRewardsBucket = await getCompanyRewardsBucketPDA(program, companyLicensePDA);
  return await program.account.companyRewardsBucket.fetch(companyRewardsBucket);
}

export const getCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = await getCompanyLicensePDA(program, wallet);
  return await program.account.companyLicense.fetch(companyLicensePDA);
}

export const getUserRewardsBucketPDA = async (program: Program<GratieSolana>, user: anchor.web3.PublicKey) => {
  const [userRewardsBucketPDA, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('user_rewards_bucket'),
      user.toBuffer(),
    ],
    program.programId
  );
  return userRewardsBucketPDA;
}

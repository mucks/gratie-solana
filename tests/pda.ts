import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";


const getPDA = (program: Program<GratieSolana>, id: string, keys: anchor.web3.PublicKey[]) => {
  const [pda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(id),
      ...keys.map((key) => key.toBuffer()),
    ],
    program.programId
  );
  return pda;
}

export const getUserPDA = (program: Program<GratieSolana>, companyLicensePublicKey: anchor.web3.PublicKey, userAccountPublicKey: anchor.web3.PublicKey) => {
  return getPDA(program, 'user', [companyLicensePublicKey, userAccountPublicKey]);
}

export const getCompanyLicensePDA = (program: Program<GratieSolana>, wallet: Wallet) => {
  return getPDA(program, 'company_license', [wallet.publicKey]);
}

export const getCompanyRewardsBucketPDA = (program: Program<GratieSolana>, companyLicensePDA: anchor.web3.PublicKey) => {
  return getPDA(program, 'company_rewards_bucket', [companyLicensePDA]);
}

export const getUserRewardsBucketPDA = (program: Program<GratieSolana>, user: anchor.web3.PublicKey) => {
  return getPDA(program, 'user_rewards_bucket', [user]);
}

export const getCompanyRewardsBucket = async (program: Program<GratieSolana>, companyLicensePDA: anchor.web3.PublicKey) => {
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);
  return await program.account.companyRewardsBucket.fetch(companyRewardsBucketPDA);
}

export const getCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, wallet);
  return await program.account.companyLicense.fetch(companyLicensePDA);
}


import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";


const getPDA = (program: Program<GratieSolana>, id: string, keys: (anchor.web3.PublicKey | string | number)[]) => {
  const [pda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(id),
      ...keys.map((key) => {
        if (typeof key === 'string') {
          return anchor.utils.bytes.utf8.encode(key);
        } else if (typeof key === 'number') {
          if (key < 0) {
            throw new Error('negative number');
          }

          if (key <= 255) {
            return new Uint8Array([key]);
          }

          throw new Error('number too big');
        }
        else {
          return key.toBuffer()
        }
      }),
    ],
    program.programId
  );
  return pda;
}

export const getTierPDA = (program: Program<GratieSolana>, tierID: number) => {
  return getPDA(program, 'tier', [tierID]);
}

export const getUserPDA = (program: Program<GratieSolana>, companyLicensePublicKey: anchor.web3.PublicKey, user_id: string) => {
  return getPDA(program, 'user', [companyLicensePublicKey, user_id]);
}

export const getCompanyLicensePDA = (program: Program<GratieSolana>, company_name: string) => {
  return getPDA(program, 'company_license', [company_name]);
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

export const getCompanyLicense = async (program: Program<GratieSolana>, company_name: string) => {
  const companyLicensePDA = getCompanyLicensePDA(program, company_name);
  return await program.account.companyLicense.fetch(companyLicensePDA);
}


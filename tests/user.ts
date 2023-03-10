import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import AES from "crypto-js/aes";
import { enc } from "crypto-js";
import { base64 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { getCompanyLicense, getCompanyLicensePDA, getCompanyRewardsBucket, getCompanyRewardsBucketPDA, getGratieWalletPDA, getTierPDA, getUserPDA, getUserRewardsBucketPDA, getUserRewardsBucketTokenAccountPDA } from "./pda";
import { GratieSolana } from "../target/types/gratie_solana";
import { COMPANY_NAME, ENCRYPTED_USER_PASSWORD, USER_EMAIL, USER_ID } from "./gratie-solana";
import { sha256 } from "@project-serum/anchor/dist/cjs/utils";
import { createTokenAccountForMint } from "./util";
import { expect } from "chai";
import * as argon2 from "argon2";
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";


// To create a user the company needs to provide the encrypted user password the user email and the password encryption algorithm and the hash
// this can be done via api
// this way the user can just come to gratie and login and will be instantly identified
// and can use his private key

export const createUser = async (program: Program<GratieSolana>, wallet: Wallet) => {

  const user = anchor.web3.Keypair.generate();

  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyLicense = await program.account.companyLicense.fetch(companyLicensePDA);
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);

  // encrypt the private key with the encrypted user password and the user id
  const encKey = sha256.hash(ENCRYPTED_USER_PASSWORD + USER_ID);
  const privKey = base64.encode(Buffer.from(user.secretKey));

  console.log("privKey: ", privKey);

  const encryptedPrivKey = AES.encrypt(privKey, encKey);

  console.log("encryptedPrivKey: ", encryptedPrivKey.toString());

  // 0 = argon2i (1, 16, 2, 16), TODO: add more encryption algorithms
  // maybe also add more options for the config to the account
  const userPasswordEncryptionAlgorithm = 0;

  const userPasswordEncryptionSalt = "saltsalt";



  await program.methods.createUser(COMPANY_NAME, USER_ID, encryptedPrivKey.toString(), userPasswordEncryptionAlgorithm, userPasswordEncryptionSalt).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    tier: companyLicense.tier,
    userAccount: user.publicKey,
    user: userPDA,
  }).rpc();

  return user.publicKey;
};

// the user needs to have the encrypted user password to decrypt the private key
export const userGetPrivateKey = async (program: Program<GratieSolana>, userPasswordPlaintext: string) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);

  const email_sha = sha256.hash(USER_EMAIL);
  const userId = email_sha.substring(0, 16);
  const userPDA = getUserPDA(program, companyLicensePDA, userId);
  const user = await program.account.user.fetch(userPDA);


  // there's gonna be a lot of coading required to support different encryption algorithms
  // maybe store these config settings as json in the user account
  const encryptedUserPassword = await argon2.hash(userPasswordPlaintext,
    {
      salt: Buffer.from(user.userPasswordSalt),
      memoryCost: 1024,
      timeCost: 2,
      hashLength: 16,
      parallelism: 1,
      type: argon2.argon2i,
    });

  console.log("encryptedUserPassword: ", encryptedUserPassword);

  const encKey = sha256.hash(encryptedUserPassword + userId);

  const decryptedPrivKey = AES.decrypt(user.encryptedPrivateKey, encKey).toString(enc.Utf8);

  console.log('decryptedPrivateKey:', decryptedPrivKey);

  const buffer = base64.decode(decryptedPrivKey);
  const keypair = anchor.web3.Keypair.fromSecretKey(buffer);

  expect(keypair.publicKey.toBase58).equals(user.owner.toBase58);

};

const getAllUserRewardsBuckets = async (program: Program<GratieSolana>) => {
  return await program.account.userRewardsBucket.all();
}

export const createUserRewardsBucket = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);
  const companyRewardsBucket = await getCompanyRewardsBucket(program, companyLicensePDA);
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);
  const userRewardsBucketPDA = getUserRewardsBucketPDA(program, userPDA);
  const user = await program.account.user.fetch(userPDA);
  const tokenMintPubkey = companyRewardsBucket.tokenMintKey;

  const tokenAccount = await createTokenAccountForMint(program, wallet.publicKey, tokenMintPubkey, user.owner);

  console.log("userRewardsBucketPDA: ", userRewardsBucketPDA.toBase58());


  await program.methods.createUserRewardsBucket(COMPANY_NAME, USER_ID).accounts({
    mintAuthority: wallet.publicKey,
    user: userPDA,
    companyLicense: companyLicensePDA,
    companyRewardsBucket: companyRewardsBucketPDA,
    userRewardsBucket: userRewardsBucketPDA,
    tokenAccount: tokenAccount,
  }).rpc();

}


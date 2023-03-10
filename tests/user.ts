import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import AES from "crypto-js/aes";
import { enc } from "crypto-js";
import { base64 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { getCompanyLicensePDA, getCompanyRewardsBucket, getCompanyRewardsBucketPDA, getGratieWalletPDA, getTierPDA, getUserPDA, getUserRewardsBucketPDA, getUserRewardsBucketTokenAccountPDA } from "./pda";
import { GratieSolana } from "../target/types/gratie_solana";
import { COMPANY_NAME, USER_PASSWORD_SALT, USER_PASSWORD, USER_EMAIL, USER_ID } from "./gratie-solana";
import { sha256 } from "@project-serum/anchor/dist/cjs/utils";
import { companyCreateInitialTokenAccountForUser, userCreateTokenAccount } from "./util";
import { expect } from "chai";
import * as argon2 from "argon2";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";


// To create a user the company needs to provide the encrypted user password the user email and the password encryption algorithm and the hash
// this can be done via api
// this way the user can just come to gratie and login and will be instantly identified
// and can use his private key

export const createUser = async (program: Program<GratieSolana>, wallet: Wallet) => {

  const user = anchor.web3.Keypair.generate();

  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyLicense = await program.account.companyLicense.fetch(companyLicensePDA);
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);

  const encryptedPassword = await encryptUserPassword(USER_PASSWORD, USER_PASSWORD_SALT);

  // encrypt the private key with the encrypted user password and the user id
  const encKey = sha256.hash(encryptedPassword + USER_ID);
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

export const encryptUserPassword = async (userPasswordPlaintext: string, salt: string) => {
  return await argon2.hash(userPasswordPlaintext,
    {
      salt: Buffer.from(Buffer.from(salt)),
      memoryCost: 1024,
      timeCost: 2,
      hashLength: 16,
      parallelism: 1,
      type: argon2.argon2i,
    });
};

// the user needs to have the encrypted user password to decrypt the private key
export const userGetPrivateKey = async (program: Program<GratieSolana>, userPasswordPlaintext: string) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);

  const email_sha = sha256.hash(USER_EMAIL);
  const userId = email_sha.substring(0, 16);
  const userPDA = getUserPDA(program, companyLicensePDA, userId);
  const user = await program.account.user.fetch(userPDA);

  let encKey = "";

  if (user.claimed) {
    encKey = sha256.hash(userPasswordPlaintext + userId);
  } else {
    const encryptedUserPassword = await encryptUserPassword(userPasswordPlaintext, user.userPasswordSalt);

    // there's gonna be a lot of coading required to support different encryption algorithms
    // maybe store these config settings as json in the user account
    console.log("encryptedUserPassword: ", encryptedUserPassword);

    encKey = sha256.hash(encryptedUserPassword + userId);

  }

  const decryptedPrivKey = AES.decrypt(user.encryptedPrivateKey, encKey).toString(enc.Utf8);

  console.log('decryptedPrivateKey:', decryptedPrivKey);

  const buffer = base64.decode(decryptedPrivKey);
  const keypair = anchor.web3.Keypair.fromSecretKey(buffer);

  expect(keypair.publicKey.toBase58).equals(user.owner.toBase58);

  return keypair;

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

  const tokenAccount = await companyCreateInitialTokenAccountForUser(program, wallet.publicKey, tokenMintPubkey, user.owner);

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


// claims the user to a new private key that is generated here
export const claimUser = async (program: Program<GratieSolana>, userPassword: string) => {
  const newUserKeypair = anchor.web3.Keypair.generate();

  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucket = await getCompanyRewardsBucket(program, companyLicensePDA);
  const tokenMintPubkey = companyRewardsBucket.tokenMintKey;
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);
  const user = await program.account.user.fetch(userPDA);

  if (user.claimed) {
    throw new Error("User already claimed");
  }


  const userRewardsBucketPDA = getUserRewardsBucketPDA(program, userPDA);
  const userRewardsBucket = await program.account.userRewardsBucket.fetch(userRewardsBucketPDA);


  const encKey = sha256.hash(userPassword + USER_ID);
  const privKey = base64.encode(Buffer.from(newUserKeypair.secretKey));

  const encryptedPrivateKey = AES.encrypt(privKey, encKey).toString();

  const oldUser = await userGetPrivateKey(program, userPassword);

  const tokenAccount = await userCreateTokenAccount(program, oldUser, tokenMintPubkey, newUserKeypair.publicKey);

  console.log("newUserKeypair.publicKey: ", newUserKeypair.publicKey.toBase58());
  console.log('newTokenAccount:', tokenAccount.toBase58());

  //TODO: add the signer to anchor somehow
  // so that the tests can sign with diffrent accounts


  console.log('SENDING CLAIM USER TX');

  await program.methods.claimUser(encryptedPrivateKey).accounts({
    claimer: user.owner,
    user: userPDA,
    userRewardsBucket: userRewardsBucketPDA,
    userAccount: newUserKeypair.publicKey,
    oldTokenAccount: userRewardsBucket.tokenAccount,
    newTokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).signers([oldUser]).rpc();

};

export const claimUserToHisOwnWallet = async (program: Program<GratieSolana>, newUserPubKey: anchor.web3.PublicKey, userPassword: string) => {

  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucket = await getCompanyRewardsBucket(program, companyLicensePDA);
  const tokenMintPubkey = companyRewardsBucket.tokenMintKey;
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);
  const user = await program.account.user.fetch(userPDA);

  if (!user.claimed) {
    throw new Error("User not claimed yet, user needs to be claimed before being able to claim to his own wallet");
  }

  if (user.claimedToHisOwnWallet) {
    throw new Error("User already claimed to his own wallet");
  }


  const userRewardsBucketPDA = getUserRewardsBucketPDA(program, userPDA);
  const userRewardsBucket = await program.account.userRewardsBucket.fetch(userRewardsBucketPDA);

  const oldUser = await userGetPrivateKey(program, userPassword);

  // create tokenaccount with fees from olduser account and associat token to new account
  const tokenAccount = await userCreateTokenAccount(program, oldUser, tokenMintPubkey, newUserPubKey);


  await program.methods.claimUserToHisOwnWallet(
    newUserPubKey,
  ).accounts({
    claimer: user.owner,
    user: userPDA,
    userAccount: newUserPubKey,
    userRewardsBucket: userRewardsBucketPDA,
    oldTokenAccount: userRewardsBucket.tokenAccount,
    newTokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).signers([oldUser]).rpc();

};
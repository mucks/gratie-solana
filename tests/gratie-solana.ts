import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { expect } from "chai";
import { createMintKeyAndTokenAccount, createTokenAccountForMint } from "./util";
import { getCompanyLicense, getCompanyLicensePDA, getCompanyRewardsBucket, getCompanyRewardsBucketPDA, getGratieWalletPDA, getTierPDA, getUserPDA, getUserRewardsBucketPDA } from "./pda";
import { createTier } from "./tier";
import { faker } from '@faker-js/faker';
import { sha256 } from "@project-serum/anchor/dist/cjs/utils";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

// THIS needs to be unique!
const COMPANY_NAME = faker.company.name();
// userID could be a sha of the user email to help identify them
const USER_EMAIL = faker.internet.email();
const email_sha = sha256.hash(USER_EMAIL);
const USER_ID = email_sha.substring(0, 16);

describe("gratie-solana", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GratieSolana as Program<GratieSolana>
  const wallet = anchor.AnchorProvider.env().wallet as Wallet;

  it("create-gratie-wallet", async () => {
    try {
      await createGratieWallet(program, wallet);
    } catch (e) {
      // this means it has already been created
      if (e.message.includes("custom program error: 0x0")) {
        return;
      }
    }
  });

  it('log-gratie-wallet', async () => {
    const gratieWalletPDA = getGratieWalletPDA(program);
    const gratieWallet = await program.account.gratieWallet.fetch(gratieWalletPDA);

    console.log("GratieWalletPDA: ", gratieWalletPDA.toBase58());
    console.log("GratieWallet: ", gratieWallet);
  });


  it("create-tier", async () => {
    try {
      await createTier(program, wallet.publicKey);
    } catch (e) {
      // this means it has already been created
      if (e.message.includes("custom program error: 0x0")) {
        return;
      }
    }
  });

  it('create-company-license', async () => {
    await createCompanyLicense(program, wallet);
  });

  it('withdraw-from-gratie-wallet', async () => {
    await withdrawFromGratieWallet(program, wallet, 1 * LAMPORTS_PER_SOL);
  });


  it('verify-company-license', async () => {
    await verifyCompanyLicense(program, wallet);
  });

  it('create-company-rewards', async () => {
    await createCompanyRewardsBucket(program, wallet);
  });


  it('create-user', async () => {
    await createUser(program, wallet);
  });

  it('create-user-rewards-bucket', async () => {
    await createUserRewardsBucket(program, wallet);
  });

  it('transfer-tokens-to-user', async () => {
    // transfer 5 tokens to user
    const amount = new anchor.BN(5);
    await transferTokensToUser(program, wallet, amount);
  });



  // it("mint-company-license-metaplex", async () => {
  //   await testMintCompanyLicenseMetaplex(program, wallet);
  // });
});


const withdrawFromGratieWallet = async (program: Program<GratieSolana>, wallet: Wallet, amount: number) => {
  const gratieWalletPDA = getGratieWalletPDA(program);

  await program.methods.withdrawFromGratieWallet(new anchor.BN(amount)).accounts({
    withdrawer: wallet.publicKey,
    gratieWallet: gratieWalletPDA,
  }).rpc();
};


const createGratieWallet = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const gratieWalletPDA = getGratieWalletPDA(program);
  await program.methods.createGratieWallet().accounts({
    owner: wallet.publicKey,
    gratieWallet: gratieWalletPDA,
  }).rpc();
};

const transferTokensToUser = async (program: Program<GratieSolana>, wallet: Wallet, amount: anchor.BN) => {
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


const getAllUserRewardsBuckets = async (program: Program<GratieSolana>) => {
  return await program.account.userRewardsBucket.all();
}


const createUser = async (program: Program<GratieSolana>, wallet: Wallet) => {

  //TODO:  probably have to add this keypair to chain before or something
  const user = anchor.web3.Keypair.generate();

  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyLicense = await program.account.companyLicense.fetch(companyLicensePDA);
  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);


  // TODO: encrypt this with the companys public key and the user email and the users hashed password
  // companies have this user data usually on their database
  // INFO: even encrypted like this the company will still have full access to the bucket
  // TODO: the user needs to be notified about that and asked to change the encryption when using the bucket
  // also user password changes will cause issues with this
  // also be encryted by userId
  const encryptedPrivateKey = user.secretKey.toString();


  await program.methods.createUser(COMPANY_NAME, USER_ID, encryptedPrivateKey).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    tier: companyLicense.tier,
    userAccount: user.publicKey,
    user: userPDA,
  }).rpc();

  return user.publicKey;
};

const createUserRewardsBucket = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);
  const companyRewardsBucket = await getCompanyRewardsBucket(program, companyLicensePDA);
  const tokenMintPubkey = companyRewardsBucket.tokenMintKey;

  const userPDA = getUserPDA(program, companyLicensePDA, USER_ID);
  const user = await program.account.user.fetch(userPDA);
  const userRewardsBucketPDA = getUserRewardsBucketPDA(program, userPDA);

  const tokenAccount = await createTokenAccountForMint(program, wallet.publicKey, tokenMintPubkey, user.owner);


  await program.methods.createUserRewardsBucket(COMPANY_NAME).accounts({
    mintAuthority: wallet.publicKey,
    user: userPDA,
    companyLicense: companyLicensePDA,
    companyRewardsBucket: companyRewardsBucketPDA,
    userRewardsBucket: userRewardsBucketPDA,
    tokenAccount: tokenAccount,
  }).rpc();


}

const createCompanyRewardsBucket = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);
  const companyRewardsBucketPDA = getCompanyRewardsBucketPDA(program, companyLicensePDA);

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet.publicKey);


  await program.methods.createCompanyRewardsBucket(COMPANY_NAME).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    companyRewardsBucket: companyRewardsBucketPDA,
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).rpc();

}


const verifyCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicense = getCompanyLicensePDA(program, COMPANY_NAME);
  await program.methods.verifyCompanyLicense().accounts({ admin: wallet.publicKey, companyLicense: companyLicense }).rpc();


  const updatedLicense = await getCompanyLicense(program, COMPANY_NAME);
  expect(updatedLicense.verified).to.equal(true);
}

const createCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = getCompanyLicensePDA(program, COMPANY_NAME);

  const testEmail = "mail@mucks.dev";
  const testLogoUri = "https://v2.akord.com/public/vaults/active/G8DOVyi_zmdssZVa6NFY5K1gKIKVW9q7gyXGhVltbsI/gallery#public/74959dec-5113-4b8b-89a0-a1e56ce8d89e";
  const testEvaluation = new anchor.BN(100000);
  const tierID = 0;
  const tierPDA = getTierPDA(program, tierID);
  const tier = await program.account.tier.fetch(tierPDA);


  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet.publicKey);

  const gratieWalletBefore = await program.account.gratieWallet.fetch(getGratieWalletPDA(program));
  const oldAmountEarned = gratieWalletBefore.amountEarned.toNumber();


  await program.methods.createCompanyLicense(COMPANY_NAME, testEmail, testLogoUri, testEvaluation, tierID).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    gratieWallet: getGratieWalletPDA(program),
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    tier: tierPDA,
  }).rpc();

  const companyLicense = await getCompanyLicense(program, COMPANY_NAME);
  expect(companyLicense.name).to.equal(COMPANY_NAME);

  // check if the amountEarned increased by the price of the tier
  const gratieWallet = await program.account.gratieWallet.fetch(getGratieWalletPDA(program));
  const amountEarnedDiff = gratieWallet.amountEarned.toNumber() - oldAmountEarned;
  expect(amountEarnedDiff).to.equal(tier.priceLamports.toNumber());
}



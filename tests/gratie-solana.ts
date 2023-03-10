import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { createTier } from "./tier";
import { faker } from '@faker-js/faker';
import { sha256 } from "@project-serum/anchor/dist/cjs/utils";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { claimUser, claimUserToHisOwnWallet, createUser, createUserRewardsBucket, userGetPrivateKey } from "./user";
import { getGratieWalletPDA } from "./pda";
import { createGratieWallet, withdrawFromGratieWallet } from "./gratieWallet";
import { transferTokensToUser } from "./transfer";
import { createCompanyLicense, createCompanyRewardsBucket, verifyCompanyLicense } from "./company";

//NOTE: currently my wallet is the creator of the gratie wallet and the company license
//      should add an extra company wallet and sign some of the transactions with that


// THIS needs to be unique!
export const COMPANY_NAME = faker.company.name();
// userID could be a sha of the user email to help identify them
export const USER_EMAIL = faker.internet.email();
const email_sha = sha256.hash(USER_EMAIL);
export const USER_ID = email_sha.substring(0, 16);

// this is for testing the company does not need to provide
// any plaintext passwords
// they need to provide the encrypted password the salt and the hashing algorithm
export const USER_PASSWORD = "helloworld";
export const USER_PASSWORD_SALT = "saltsalt";


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

  it('user-get-private-key', async () => {
    await userGetPrivateKey(program, "helloworld");
  });

  it('create-user-rewards-bucket', async () => {
    await createUserRewardsBucket(program, wallet);
  });

  it('transfer-tokens-to-user', async () => {
    // transfer 5 tokens to user
    const amount = new anchor.BN(5);
    await transferTokensToUser(program, wallet, amount);
  });

  // This should be called when the user logs in the first time to claim their account automatically
  it('claim-user', async () => {
    await claimUser(program, wallet, USER_PASSWORD);
  });

  it('claim-user-own-wallet', async () => {
    // test public key get this one from phantom wallet on application
    const userPubKey = anchor.web3.Keypair.generate().publicKey;

    await claimUserToHisOwnWallet(program, wallet, userPubKey, USER_PASSWORD);
  });



  // it("mint-company-license-metaplex", async () => {
  //   await testMintCompanyLicenseMetaplex(program, wallet);
  // });
});



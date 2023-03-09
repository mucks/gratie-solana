import * as anchor from "@project-serum/anchor";
import { Program, Wallet, AnchorProvider, SystemProgram } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'
import { expect } from "chai";
import { createMintKeyAndTokenAccount, createTokenAccountForMint } from "./util";
import { getCompanyLicense, getCompanyLicensePDA, getCompanyRewardsBucket, getCompanyRewardsBucketPDA, getUserPDA, getUserRewardsBucketPDA } from "./pda";


// THIS needs to be unique!
const COMPANY_NAME = "test_company_4";
// userID could be a sha of the user email to help identify them
const USER_ID = "b02b64a0-f570-40aF";

describe("gratie-solana", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GratieSolana as Program<GratieSolana>
  const wallet = anchor.AnchorProvider.env().wallet as Wallet;


  it('create-company-license', async () => {
    await createCompanyLicense(program, wallet);
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

  const companyLicense = getCompanyLicensePDA(program, COMPANY_NAME);
  const userPDA = getUserPDA(program, companyLicense, USER_ID);

  const testUserEmail = "test-user@mucks.dev";

  // TODO: encrypt this with the companys public key and the user email and the users hashed password
  // companies have this user data usually on their database
  // INFO: even encrypted like this the company will still have full access to the bucket
  // TODO: the user needs to be notified about that and asked to change the encryption when using the bucket
  // also user password changes will cause issues with this
  // also be encryted by userId
  const encryptedPrivateKey = user.secretKey.toString();


  await program.methods.createUser(COMPANY_NAME, USER_ID, encryptedPrivateKey).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicense,
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

  console.log('CREATED USER REWARDS BUCKET');

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
  const tier = 0;

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet.publicKey);

  console.log("CREATED COMPANY LICENSE TOKEN ACCOUNTS");

  await program.methods.createCompanyLicense(COMPANY_NAME, testEmail, testLogoUri, testEvaluation, tier).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).rpc();

  const companyLicense = await getCompanyLicense(program, COMPANY_NAME);

  console.log(companyLicense);

  expect(companyLicense.name).to.equal(COMPANY_NAME);
}


// Note: this works on devnet but not on localnet
const testMintCompanyLicenseMetaplex = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const lamports: number =
    await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

  const getMetadata = async (
    mint: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  const getMasterEdition = async (
    mint: anchor.web3.PublicKey
  ): Promise<anchor.web3.PublicKey> => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const NftTokenAccount = await getAssociatedTokenAddress(
    mintKey.publicKey,
    wallet.publicKey
  );
  console.log("NFT Account: ", NftTokenAccount.toBase58());


  const mint_tx = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mintKey.publicKey,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
      lamports,
    }),
    createInitializeMintInstruction(
      mintKey.publicKey,
      0,
      wallet.publicKey,
      wallet.publicKey
    ),
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      NftTokenAccount,
      wallet.publicKey,
      mintKey.publicKey
    )
  );

  const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
  console.log(
    await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
  );

  console.log("Account: ", res);
  console.log("Mint key: ", mintKey.publicKey.toString());
  console.log("User: ", wallet.publicKey.toString());

  const metadataAddress = await getMetadata(mintKey.publicKey);
  const masterEdition = await getMasterEdition(mintKey.publicKey);

  console.log("Metadata address: ", metadataAddress.toBase58());
  console.log("MasterEdition: ", masterEdition.toBase58());

  // Transaction error 0xb can happen if uri and name are swapped
  const tx = await program.methods.mintCompanyLicenseMetaplex(
    // creator
    mintKey.publicKey,
    // uri
    "https://minio.mucks.dev/public/company-license-sample.json",
    // name
    "Gratie Sample",
  )
    .accounts({
      mintAuthority: wallet.publicKey,
      mint: mintKey.publicKey,
      tokenAccount: NftTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      metadata: metadataAddress,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      payer: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      masterEdition: masterEdition,
    },
    )
    .rpc();

  console.log("Your transaction signature", tx);
};


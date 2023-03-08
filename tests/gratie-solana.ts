import * as anchor from "@project-serum/anchor";
import { Program, Wallet, AnchorProvider, SystemProgram } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'
import { expect } from "chai";
import { createMintKeyAndTokenAccount } from "./util";


describe("gratie-solana", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GratieSolana as Program<GratieSolana>
  const wallet = anchor.AnchorProvider.env().wallet as Wallet;

  it('get-company-license', async () => {
    const companyLicense = await getCompanyLicense(program, wallet);
    console.log(companyLicense);
  });

  it('create-company-license', async () => {
    await createCompanyLicense(program, wallet);
  });

  it('create-company-rewards', async () => {
    await createCompanyRewards(program, wallet);
  });

  it('verify-company-license', async () => {
    await verifyCompanyLicense(program, wallet);
  });

  it('get-user-reward-buckets', async () => {
  });

  it('create-user-rewards-bucket', async () => {
    await testCreateUserRewardsBucket(program, wallet);
  });

  // this requires devnet for now because of the metaplex program
  // it('get-metadata', async () => {
  //   await testGetMetadata(program, wallet);
  // });

  // it("mint-company-license-metaplex", async () => {
  //   await testMintCompanyLicenseMetaplex(program, wallet);
  // });
});

const testCreateUserRewardsBucket = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicense = await getCompanyLicensePDA(program, wallet);
  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet);
  const user = anchor.web3.Keypair.generate();

  const userRewardsBucketPDA = await getUserRewardsBucketPDA(program, wallet, user.publicKey);

  // TODO: encrypt this with the companys public key and the user email and the users hashed password
  // companies have this user data usually on their database
  // INFO: even encrypted like this the company will still have full access to the bucket
  // TODO: the user needs to be notified about that and asked to change the encryption when using the bucket
  // also user password changes will cause issues with this
  const encryptedPrivateKey = user.secretKey.toString();
  const testUserEmail = "test-user@mucks.dev";

  await program.methods.createUserRewardsBucket(testUserEmail, encryptedPrivateKey).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicense,
    userRewardsBucket: userRewardsBucketPDA,
    userAccount: user.publicKey,
    tokenAccount: tokenAccount,
  }).rpc();


}

const createCompanyRewards = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = await getCompanyLicensePDA(program, wallet);

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet);


  await program.methods.createCompanyRewards().accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).rpc();

}


const verifyCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicense = await getCompanyLicensePDA(program, wallet);
  await program.methods.verifyCompanyLicense().accounts({ admin: wallet.publicKey, companyLicense: companyLicense }).rpc();


  const updatedLicense = await getCompanyLicense(program, wallet);
  expect(updatedLicense.verified).to.equal(true);
}

const getUserRewardsBucketPDA = async (program: Program<GratieSolana>, wallet: Wallet, user: anchor.web3.PublicKey) => {
  const [userRewardsBucketPDA, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('user_rewards_bucket'),
      wallet.publicKey.toBuffer(),
      user.toBuffer(),
    ],
    program.programId
  );
  return userRewardsBucketPDA;
}

const getCompanyLicensePDA = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const [companyLicensePDA, _] = await anchor.web3.PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode('company_license'),
      wallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  return companyLicensePDA;
}

const getCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = await getCompanyLicensePDA(program, wallet);
  return await program.account.companyLicense.fetch(companyLicensePDA);
}

const createCompanyLicense = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const companyLicensePDA = await getCompanyLicensePDA(program, wallet);

  const testName = "MucksCompany";
  const testEmail = "mail@mucks.dev";
  const testLogoUri = "https://v2.akord.com/public/vaults/active/G8DOVyi_zmdssZVa6NFY5K1gKIKVW9q7gyXGhVltbsI/gallery#public/74959dec-5113-4b8b-89a0-a1e56ce8d89e";
  const testEvaluation = new anchor.BN(100000);
  const tier = 0;

  const { mintKey, tokenAccount } = await createMintKeyAndTokenAccount(program, wallet);


  await program.methods.createCompanyLicense(testName, testEmail, testLogoUri, testEvaluation, tier).accounts({
    mintAuthority: wallet.publicKey,
    companyLicense: companyLicensePDA,
    mint: mintKey.publicKey,
    tokenAccount: tokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  }).rpc();

  const companyLicense = await getCompanyLicense(program, wallet);

  expect(companyLicense.name).to.equal(testName);
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


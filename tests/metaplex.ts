import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'
import { getCompanyLicense, getCompanyLicensePDA } from "./pda";

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


export const addCompanyLicenseToMetaplex = async (program: Program<GratieSolana>, wallet: Wallet, companyName: string) => {
  const companyLicense = await getCompanyLicense(program, companyName);

  const metadataAddress = getMetadata(companyLicense.mint);
  const masterEdition = getMasterEdition(companyLicense.mint);

  // Transaction error 0xb can happen if uri and name are swapped
  await program.methods.addCompanyLicenseToMetaplex()
    .accounts({
      companyLicenseOwner: wallet.publicKey,
      companyLicense: getCompanyLicensePDA(program, companyName),
      mint: companyLicense.mint,
      tokenAccount: companyLicense.tokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      metadata: metadataAddress,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      masterEdition: masterEdition,
    },
    )
    .rpc();


};


export const addCompanyRewardTokensToMetaplex = async (program: Program<GratieSolana>, wallet: Wallet, companyName: string) => {
  const companyLicense = await getCompanyLicense(program, companyName);

  const metadataAddress = getMetadata(companyLicense.mint);

  // Transaction error 0xb can happen if uri and name are swapped
  await program.methods.addCompanyRewardTokensToMetaplex()
    .accounts({
      companyLicenseOwner: wallet.publicKey,
      companyLicense: getCompanyLicensePDA(program, companyName),
      mint: companyLicense.mint,
      tokenAccount: companyLicense.tokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      metadata: metadataAddress,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    },
    )
    .rpc();
};

const getMetadata = (mint: anchor.web3.PublicKey) => {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  )[0];
};

const getMasterEdition = (mint: anchor.web3.PublicKey) => {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID
  )[0];
};

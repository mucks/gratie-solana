import * as anchor from "@project-serum/anchor";
import { Program, Wallet, SystemProgram } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'


export const createTokenAccountForMint = async (program: Program<GratieSolana>, walletPublicKey: anchor.web3.PublicKey, mintKey: anchor.web3.PublicKey, user: anchor.web3.PublicKey) => {
  const tokenAccount = await getAssociatedTokenAddress(
    mintKey,
    user
  );

  const tx = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      walletPublicKey,
      tokenAccount,
      user,
      mintKey
    )
  );

  await program.provider.sendAndConfirm(tx, []);

  return tokenAccount;

};

export const createMintKeyAndTokenAccount = async (program: Program<GratieSolana>, walletPublicKey: anchor.web3.PublicKey) => {
  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  const tokenAccount = await getAssociatedTokenAddress(
    mintKey.publicKey,
    walletPublicKey
  );

  // TODO: move this to another function
  const lamports: number =
    await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

  const mint_tx = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: walletPublicKey,
      newAccountPubkey: mintKey.publicKey,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
      lamports,
    }),
    createInitializeMintInstruction(
      mintKey.publicKey,
      0,
      walletPublicKey,
      walletPublicKey
    ),
    createAssociatedTokenAccountInstruction(
      walletPublicKey,
      tokenAccount,
      walletPublicKey,
      mintKey.publicKey
    )
  );

  await program.provider.sendAndConfirm(mint_tx, [mintKey]);

  return { mintKey, tokenAccount };
}
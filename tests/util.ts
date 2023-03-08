import * as anchor from "@project-serum/anchor";
import { Program, Wallet, SystemProgram } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'

export const createMintKeyAndTokenAccount = async (program: Program<GratieSolana>, wallet: Wallet) => {
  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  const tokenAccount = await getAssociatedTokenAddress(
    mintKey.publicKey,
    wallet.publicKey
  );

  // TODO: move this to another function
  const lamports: number =
    await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

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
      tokenAccount,
      wallet.publicKey,
      mintKey.publicKey
    )
  );

  await program.provider.sendAndConfirm(mint_tx, [mintKey]);

  return { mintKey, tokenAccount };
}
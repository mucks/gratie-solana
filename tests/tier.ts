import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { getTierPDA } from "./pda";

export const createTier = async (program: Program<GratieSolana>, walletPubKey: anchor.web3.PublicKey) => {
  const tierID = 1;
  const name = "fractal";
  const freeUserAmount = 500;
  const price = new BN(3 * LAMPORTS_PER_SOL);
  const priceForEachAdditionalUser = new BN(LAMPORTS_PER_SOL);
  // 15 is 1.5% here
  const platformFeePerMille = 15;

  const tierPDA = getTierPDA(program, tierID);

  console.log("tierPDA", tierPDA.toBase58());

  await program.methods.createTier(
    tierID,
    name,
    freeUserAmount,
    price,
    priceForEachAdditionalUser,
    platformFeePerMille,
  ).accounts({
    creator: walletPubKey,
    tier: tierPDA,
  }).rpc();

}
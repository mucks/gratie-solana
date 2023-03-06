import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { GratieSolana } from "../target/types/gratie_solana";

describe("gratie-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GratieSolana as Program<GratieSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

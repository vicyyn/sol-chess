import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolChess } from "../target/types/sol_chess";

describe("sol-chess", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolChess as Program<SolChess>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MultisigSol } from "../target/types/multisig_sol";

describe("multisig-sol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MultisigSol as Program<MultisigSol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

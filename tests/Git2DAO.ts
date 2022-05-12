import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Git2Dao } from "../target/types/git2_dao";

describe("Git2DAO", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Git2Dao as Program<Git2Dao>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

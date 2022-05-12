import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Git2Dao } from "../target/types/git2_dao";

import { expect } from 'chai';

describe("Git2DAO", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Git2Dao as Program<Git2Dao>;
  const programProvider = program.provider as anchor.AnchorProvider;

  it("dao created!", async () => {
    const dao_keypair = anchor.web3.Keypair.generate();
    const repo_keypair = programProvider.wallet;

    const repo_link = "https://github.com/never2average/Git2DAO-a3e66fb81e1e6fda50ae1ee1dab8579936f72c7b6c06b42b30f8882485c87170";

    const tx = await program.methods.create_Dao(repo_link)
               accounts({
                 dao:  dao_keypair.publicKey,
                 owner: repo_keypair.publicKey
               })
               .signers([dao_keypair])
               .rpc();

    let dao_state = await program.account.game.fetch(dao_keypair.publicKey);
    console.log(dao_state);

    // console.assert(repo_link === String.fromCharCode.apply(null,dao_state.repo_url),"unexpected repo_link");

    // console.assert(owner.publicKey.to === String.fromCharCode.apply(null,dao_state.repo_url),"unexpected repo_link");


    // console.log("Your transaction signature", tx);
  });
});

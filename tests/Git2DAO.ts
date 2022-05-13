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
    
    const repo_link = "https://github.com/never2average/Git2DAO-"+byteToHexString(repo_keypair.publicKey.toBytes());

    await program.methods.createDao(repo_link)
               .accounts({
                 dao:  dao_keypair.publicKey,
                 owner: repo_keypair.publicKey
               })
               .signers([dao_keypair])
               .rpc();

    let dao_state = await program.account.dao.fetch(dao_keypair.publicKey);
    console.log(dao_state);

    console.assert(repo_link === String.fromCharCode.apply(null,dao_state.repoUrl),"unexpected repo_link");

  });
});


function byteToHexString(uint8arr) {
  if (!uint8arr) {
    return '';
  }
  
  var hexStr = '';
  for (var i = 0; i < uint8arr.length; i++) {
    var hex = (uint8arr[i] & 0xff).toString(16);
    hex = (hex.length === 1) ? '0' + hex : hex;
    hexStr += hex;
  }
  return hexStr
}
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Git2Dao } from "../target/types/git2_dao";

import { expect } from 'chai';

describe("Git2DAO", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Git2Dao as Program<Git2Dao>;
  const programProvider = program.provider as anchor.AnchorProvider;
  const repo_keypair = programProvider.wallet;

  const dao_keypair = anchor.web3.Keypair.generate();
  const issue_keypair = anchor.web3.Keypair.generate();
  const issue_raiser_keypair = anchor.web3.Keypair.generate();
  const user_keypair = anchor.web3.Keypair.generate();
  const dummy_commit_keypair = anchor.web3.Keypair.generate();

  it("dao created!", async () => {
    
    const repo_link = "https://github.com/never2average/Git2DAO-"+byteToHexString(repo_keypair.publicKey.toBytes());

    await program.methods.createDao(repo_link)
               .accounts({
                 dao:  dao_keypair.publicKey,
                 owner: repo_keypair.publicKey
               })
               .signers([dao_keypair])
               .rpc()

    const dao_state = await program.account.dao.fetch(dao_keypair.publicKey);
    console.log(dao_state);

    console.assert(repo_link === String.fromCharCode.apply(null,dao_state.repoUrl),"unexpected repo_link");

  });

  it("user registered!", async () => {

    const sign1 = await programProvider.connection.requestAirdrop(user_keypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(sign1);

    await program.methods.register()
               .accounts({
                 user:  issue_raiser_keypair.publicKey,
                 owner: user_keypair.publicKey
               })
               .signers([issue_raiser_keypair,user_keypair])
               .rpc();

    const user_state = await program.account.user.fetch(issue_raiser_keypair.publicKey);
    console.log(user_state);

  });

  it("issue raised!", async () => {

    const sign2 = await programProvider.connection.requestAirdrop(issue_raiser_keypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(sign2);

    const sol_staked = new anchor.BN(1000000);
    const issue_num = 2;

    await program.methods.raiseIssue(sol_staked,issue_num)
               .accounts({
                 issue: issue_keypair.publicKey,
                 issueRaiser: issue_raiser_keypair.publicKey,
                 owner: user_keypair.publicKey,
                 dao: dao_keypair.publicKey
               })
               .signers([issue_keypair,user_keypair])
               .rpc();

    const issue_state = await program.account.issue.fetch(issue_keypair.publicKey);
    console.log(issue_state);



  });


  it("commit added!", async () => {
    
    const tree_hash = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    const commit_hash = [20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1];
    const commit_type = {dummy: {}};

    await program.methods.addCommit(tree_hash,commit_hash,commit_type)
               .accounts({
                 commit: dummy_commit_keypair.publicKey,
                 issue: issue_keypair.publicKey,
                 dao: dao_keypair.publicKey,
                 owner: repo_keypair.publicKey
               })
               .signers([dummy_commit_keypair])
               .rpc();

    const commit_state = await program.account.commit.fetch(dummy_commit_keypair.publicKey);
    console.log(commit_state);

    const dao_state = await program.account.dao.fetch(dao_keypair.publicKey);
    console.log(dao_state);

  });

  it("issue closed!", async () => {

    await program.methods.closeIssue()
               .accounts({
                 issue: issue_keypair.publicKey,
                 issueRaiser: issue_raiser_keypair.publicKey,
                 dao: dao_keypair.publicKey,
                 owner: user_keypair.publicKey
               })
               .signers([user_keypair])
               .rpc();

    const issue_state = await program.account.issue.fetch(issue_keypair.publicKey);
    console.log(issue_state);

    const dao_state = await program.account.dao.fetch(dao_keypair.publicKey);
    console.log(dao_state);

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
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
  const prev_commit_keypair = anchor.web3.Keypair.generate();
  const dummy1_commit_keypair = anchor.web3.Keypair.generate();
  const actual_commit_keypair = anchor.web3.Keypair.generate();
  const dummy2_commit_keypair = anchor.web3.Keypair.generate();


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
    const commit_type_actual = {actual: {}};
    const commit_type_dummy = {dummy: {}};

    await program.methods.addCommit(tree_hash,commit_hash,commit_type_dummy)
               .accounts({
                 commit: prev_commit_keypair.publicKey, // make sure it refences diffrent issue
                 issue: issue_keypair.publicKey,
                 dao: dao_keypair.publicKey,
                 owner: repo_keypair.publicKey
               })
               .signers([prev_commit_keypair])
               .rpc();

    await program.methods.addCommit(tree_hash,commit_hash,commit_type_dummy)
    .accounts({
      commit: dummy1_commit_keypair.publicKey,
      issue: issue_keypair.publicKey,
      dao: dao_keypair.publicKey,
      owner: repo_keypair.publicKey
    })
    .signers([dummy1_commit_keypair])
    .rpc();

    await program.methods.addCommit(tree_hash,commit_hash,commit_type_actual)
    .accounts({
      commit: actual_commit_keypair.publicKey,
      issue: issue_keypair.publicKey,
      dao: dao_keypair.publicKey,
      owner: repo_keypair.publicKey
    })
    .signers([actual_commit_keypair])
    .rpc();

    await program.methods.addCommit(tree_hash,commit_hash,commit_type_dummy)
    .accounts({
      commit: dummy2_commit_keypair.publicKey,
      issue: issue_keypair.publicKey,
      dao: dao_keypair.publicKey,
      owner: repo_keypair.publicKey
    })
    .signers([dummy2_commit_keypair])
    .rpc();

    const prev_commit_state = await program.account.commit.fetch(prev_commit_keypair.publicKey);
    console.log(prev_commit_state);

    const d1_commit_state = await program.account.commit.fetch(dummy1_commit_keypair.publicKey);
    console.log(d1_commit_state);

    const ac_commit_state = await program.account.commit.fetch(actual_commit_keypair.publicKey);
    console.log(ac_commit_state);
    
    const d2_commit_state = await program.account.commit.fetch(dummy2_commit_keypair.publicKey);
    console.log(d2_commit_state);

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

  it("reward claimed!", async () => {

    const sign3 = await programProvider.connection.requestAirdrop(user_keypair.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(sign3);

    await program.methods.claimReward()
               .accounts({
                 issue: issue_keypair.publicKey,
                 issueRaiser: issue_raiser_keypair.publicKey,
                 dao: dao_keypair.publicKey,
                 owner: user_keypair.publicKey,
               })
               .remainingAccounts([
                 {isWritable: true,isSigner: false, pubkey: prev_commit_keypair.publicKey},
                 {isWritable: true,isSigner: false, pubkey: dummy1_commit_keypair.publicKey},
                 {isWritable: true,isSigner: false, pubkey: actual_commit_keypair.publicKey},
                 {isWritable: true,isSigner: false, pubkey: dummy2_commit_keypair.publicKey},
              ])
               .signers([user_keypair])
               .rpc();

    const issue_raiser_state = await program.account.user.fetch(issue_raiser_keypair.publicKey);
    console.log(issue_raiser_state);

    const issue_state = await program.account.issue.fetch(issue_keypair.publicKey);
    console.log(issue_state);

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
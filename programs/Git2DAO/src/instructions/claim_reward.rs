use crate::error::DaoError;
use crate::state::{
    dao::Dao,
    commit::Commit,
    issue::{Issue,IssueState},
    user::User,
};
use sha1::{Sha1, Digest};

use anchor_lang::prelude::*;

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {

    let issue = &mut ctx.accounts.issue;
    let issue_pk = issue.key();
    let issue_raiser = &mut ctx.accounts.issue_raiser;
    let issue_raiser_pk = issue_raiser.key();
    let owner = &mut ctx.accounts.owner;
    let dev_pk = owner.key();

    require!(issue.issue_state == IssueState::Closed,DaoError::InvalidClaim);

    let total_commits = ctx.remaining_accounts.len();
    require!(total_commits >= 4, DaoError::InvalidCommitChain);

    let mut acc_info;

    // verify commit
    let prev_dummy_commit = & ctx.remaining_accounts[0];
    let dummy1 = & ctx.remaining_accounts[1];
    let dummy2 = & ctx.remaining_accounts[total_commits-1];
    let last_actual_commit = & ctx.remaining_accounts[total_commits - 2];

    for i in [0,1,total_commits-2,total_commits-1].iter(){
        acc_info = & ctx.remaining_accounts[*i];
        require!(acc_info.is_writable, DaoError::InvalidPermission);
        require_keys_eq!(*acc_info.owner,*ctx.program_id,DaoError::InvalidProgramOwner);
        // require!(acc_info.data_len() == Commit::LEN, DaoError::NotCommitDataStructure);
    }

    require!(dummy1.data.borrow()[8..40] == issue_pk.to_bytes(),DaoError::CommitAlreadyPaid);
    require!(dummy2.data.borrow()[8..40] == issue_pk.to_bytes(),DaoError::CommitAlreadyPaid);
    require!(last_actual_commit.data.borrow()[8..40] == issue_pk.to_bytes(),DaoError::CommitAlreadyPaid);

    require!(prev_dummy_commit.data.borrow()[Commit::LEN - 1] == 0,DaoError::InvalidCommitChain);
    require!(dummy1.data.borrow()[Commit::LEN - 1] == 0,DaoError::InvalidCommitChain);
    require!(last_actual_commit.data.borrow()[Commit::LEN - 1] == 1,DaoError::InvalidCommitChain);
    require!(dummy2.data.borrow()[Commit::LEN - 1] == 0,DaoError::InvalidCommitChain);

    for i in 2..(total_commits-2){
        acc_info = & ctx.remaining_accounts[i];
        require!(acc_info.is_writable, DaoError::InvalidPermission);
        require_keys_eq!(*acc_info.owner,*ctx.program_id,DaoError::InvalidProgramOwner);
        // require!(acc_info.data_len() == Commit::LEN, DaoError::NotCommitDataStructure);   // since none in this program is of same length
        require!(acc_info.data.borrow()[Commit::LEN-1] == 1, DaoError::InvalidCommitChain);
        require!(acc_info.data.borrow()[8..40] == issue_pk.to_bytes(),DaoError::CommitAlreadyPaid);
        *acc_info.data.borrow_mut() = &mut [];
        *(*owner.to_account_info().lamports.borrow_mut()) = owner.to_account_info().lamports() +  *(*acc_info.lamports.borrow());
        *(*acc_info.lamports.borrow_mut()) = 0;
    }

// check hashing logic

    // let mut hasher = Sha1::new();
    // let msg = [& dummy2.data.borrow()[40..60],& owner.key().to_bytes()[..]].concat();
    // hasher.update(msg);
    // let result_hash = hasher.finalize();

    // require!(result_hash[..] == last_actual_commit.data.borrow()[40..60],DaoError::InvalidClaim);

    *last_actual_commit.data.borrow_mut() = &mut [];
    *(*owner.to_account_info().lamports.borrow_mut()) = owner.to_account_info().lamports() +  *(*last_actual_commit.lamports.borrow());
    *(*last_actual_commit.lamports.borrow_mut()) = 0;

    let sol_staked = issue.sol_staked;
    *(*issue_raiser.to_account_info().lamports.borrow_mut()) = issue_raiser.to_account_info().lamports() + sol_staked;
    *(*issue.to_account_info().lamports.borrow_mut()) = issue.to_account_info().lamports() - sol_staked;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut,has_one = issue_raiser,has_one = dao)]
    pub issue: Account<'info, Issue>,
    #[account(mut,has_one = owner)]
    pub issue_raiser: Account<'info, User>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

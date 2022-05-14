use anchor_lang::prelude::*;
use crate::state::{dao::Dao,issue::Issue,commit::{Commit,CommitType}};

pub fn add_commit(
    ctx: Context<AddCommit>,
    tree_hash: [u8; 20],
    commit_hash: [u8; 20],
    commit_type: CommitType,
) -> Result<()> {
    let commit = &mut ctx.accounts.commit;
    let issue = ctx.accounts.issue.key();
    commit.add_commit(issue,tree_hash,commit_hash,commit_type);
    Ok(())
}

#[derive(Accounts)]
pub struct AddCommit<'info> {
    #[account(init, payer = owner, space = Dao::LEN)]
    pub commit: Account<'info, Commit>,
    #[account(mut,has_one = dao)]
    pub issue: Account<'info, Issue>,
    #[account(mut,has_one = owner)]
    pub dao: Account<'info,Dao>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

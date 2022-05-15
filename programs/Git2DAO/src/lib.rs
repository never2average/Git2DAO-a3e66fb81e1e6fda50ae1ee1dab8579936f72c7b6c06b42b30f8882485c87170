use anchor_lang::prelude::*;
use instructions::*;
use state::commit::CommitType;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod git2_dao {
    use super::*;

    pub fn create_dao(ctx: Context<CreateDao>, repo_url: String) -> Result<()> {
        instructions::create_dao::create_dao(ctx, repo_url)
    }

    pub fn register(ctx: Context<Register>) -> Result<()> {
        instructions::register::register(ctx)
    }

    pub fn raise_issue(ctx: Context<RaiseIssue>, sol_staked: u64,issue_num: u16) -> Result<()> {
        instructions::raise_issue::raise_issue(ctx, sol_staked,issue_num)
    }

    pub fn add_commit(
        ctx: Context<AddCommit>,
        tree_hash: [u8; 20],
        parent_hash: [u8; 20],
        commit_type: CommitType
    ) -> Result<()>{
        instructions::add_commit::add_commit(
            ctx,
            tree_hash,
            parent_hash,
            commit_type
        )
    }

    pub fn close_issue(ctx: Context<CloseIssue>) -> Result<()>{
        instructions::close_issue::close_issue(ctx)
    }
    
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()>{
        instructions::claim_reward::claim_reward(ctx)
    }
 }

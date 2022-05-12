use anchor_lang::prelude::*;
use instructions::*;

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

    pub fn raise_issue(ctx: Context<RaiseIssue>) -> Result<()> {
        instructions::raise_issue::raise_issue(ctx)
    }

    // pub fn syncState(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn submitSolution(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn pingFRT(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn stakeFRT(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    // pub fn distributeOSD(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }
}

use crate::error::DaoError;
use crate::state::{
    dao::Dao,
    issue::{Issue, IssueState},
    user::User,
};
use anchor_lang::prelude::*;

pub fn close_issue(ctx: Context<CloseIssue>) -> Result<()> {

    let issue = &mut ctx.accounts.issue;

    issue.close_issue();

    let dao = &mut ctx.accounts.dao;

    dao.dao_update_issue_closed(issue.sol_staked);

    Ok(())
}

#[derive(Accounts)]
pub struct CloseIssue<'info> {
    #[account(mut,has_one = issue_raiser,has_one = dao)]
    pub issue: Account<'info, Issue>,
    #[account(mut,has_one = owner)]
    pub issue_raiser: Account<'info, User>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account()]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

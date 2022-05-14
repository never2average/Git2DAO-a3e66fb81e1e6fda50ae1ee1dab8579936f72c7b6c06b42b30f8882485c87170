use crate::error::DaoError;
use crate::state::{
    dao::Dao,
    issue::{Issue, IssueState},
    user::User,
};
use anchor_lang::prelude::*;

pub fn raise_issue(ctx: Context<RaiseIssue>, sol_staked: u64) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let dao_pk = dao.key();
    let issue = &mut ctx.accounts.issue;
    let issue_raiser = &mut ctx.accounts.issue_raiser;

    require!(
        issue.get_issue_state() == IssueState::NotFunded,
        DaoError::IssueAlreadyRaised
    );

    require!(
        sol_staked > Issue::STAKETHRESHOLD,
        DaoError::ThresholdNotMet
    );

    let new_issue_balance = issue.to_account_infos()[0].lamports() + sol_staked;
    let issue_raiser_balance = issue_raiser.to_account_infos()[0].lamports();

    dao.dao_update_issue_raised(sol_staked).unwrap();

    *(*issue.to_account_infos()[0].lamports.borrow_mut()) = new_issue_balance;
    *(*issue_raiser.to_account_infos()[0].lamports.borrow_mut()) =
        issue_raiser_balance - sol_staked;

    let issue_num = dao.get_issue_count();

    issue.raise_issue(issue_num, dao_pk, issue_raiser.key(), sol_staked);

    Ok(())
}

#[derive(Accounts)]
pub struct RaiseIssue<'info> {
    #[account(init, payer = issue_raiser, space = Issue::LEN)]
    pub issue: Account<'info, Issue>,
    #[account(signer,mut)]
    pub issue_raiser: Account<'info, User>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

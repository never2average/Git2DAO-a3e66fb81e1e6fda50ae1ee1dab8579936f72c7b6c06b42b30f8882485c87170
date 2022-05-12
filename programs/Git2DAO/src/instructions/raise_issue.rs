use crate::error::DaoError;
use crate::state::{dao::Dao, issue::Issue};
use anchor_lang::prelude::*;

pub fn raise_issue(ctx: Context<RaiseIssue>) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let dao_pk = dao.key();
    let issue = &mut ctx.accounts.issue;
    let issue_raiser_pk = ctx.accounts.issue_raiser.key();

    let sol_staked =
        dao.to_account_infos()[0].lamports() - (dao.get_total_sol_staked() + dao.get_rent_price());

    require!(
        sol_staked > Issue::STAKETHRESHOLD,
        DaoError::ThresholdNotMet
    );

    dao.dao_update_issue_raised(sol_staked).unwrap();

    let issue_num = dao.get_issue_count();

    issue.raise_issue(issue_num, dao_pk, issue_raiser_pk, sol_staked);

    Ok(())
}

#[derive(Accounts)]
pub struct RaiseIssue<'info> {
    #[account(init, payer = issue_raiser, space = Issue::LEN)]
    pub issue: Account<'info, Issue>,
    #[account(mut)]
    pub issue_raiser: Signer<'info>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

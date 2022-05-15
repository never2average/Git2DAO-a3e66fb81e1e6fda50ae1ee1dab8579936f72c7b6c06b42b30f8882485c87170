use crate::error::DaoError;
use crate::state::{
    dao::Dao,
    issue::Issue,
    user::User,
};
use anchor_lang::prelude::*;

pub fn raise_issue(ctx: Context<RaiseIssue>, sol_staked: u64,issue_num: u16) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let dao_pk = dao.key();
    let issue = &mut ctx.accounts.issue;
    let issue_raiser = & ctx.accounts.issue_raiser;

    require!(
        sol_staked > Issue::STAKETHRESHOLD,
        DaoError::ThresholdNotMet
    );

    dao.dao_update_issue_raised(sol_staked);

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.owner.key(),
        &issue.key(),
        sol_staked,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.owner.to_account_info(),
            issue.to_account_info(),
        ],
    );

    issue.raise_issue(issue_num, dao_pk, issue_raiser.key(), sol_staked);

    Ok(())
}

#[derive(Accounts)]
pub struct RaiseIssue<'info> {
    #[account(init, payer = owner, space = Issue::LEN)]
    pub issue: Account<'info, Issue>,
    #[account(has_one = owner)]
    pub issue_raiser: Account<'info, User>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

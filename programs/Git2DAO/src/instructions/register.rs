use anchor_lang::prelude::*;

use crate::state::User;

pub fn register(ctx: Context<Register>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.register(ctx.accounts.owner.key());
    Ok(())
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(init, payer = owner, space = User::LEN)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

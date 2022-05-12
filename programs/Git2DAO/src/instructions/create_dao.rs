use crate::error::DaoError;
use crate::state::dao::Dao;
use anchor_lang::prelude::*;

pub fn create_dao(ctx: Context<CreateDao>, repo_url: String) -> Result<()> {
    let n = repo_url.len();
    require!(n <= 128, DaoError::InvalidUrlLength);

    let url_hex: [u8; 64] = repo_url.as_bytes()[n - 65..n].try_into().unwrap();

    let url_pk: [u8; 32] = hex::decode(url_hex).unwrap().try_into().unwrap();
    let url_pk = Pubkey::new_from_array(url_pk);

    let repo_owner_pk = ctx.accounts.owner.key();

    require_keys_eq!(url_pk, repo_owner_pk, DaoError::InvalidOwner);

    let dao = &mut ctx.accounts.dao;

    dao.create_dao(repo_url, repo_owner_pk);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateDao<'info> {
    #[account(init, payer = owner, space = Dao::LEN)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

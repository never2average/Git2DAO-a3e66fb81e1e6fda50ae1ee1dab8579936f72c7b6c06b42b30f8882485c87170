use anchor_lang::prelude::*;

pub type url = [u8;128];

#[account]
pub struct Dao{
    repo_url: url;              // 128
    repo_owner_pk: Pubkey,      // 32 
    open_issue_count: u16,      // 2
    // Tokens
}

impl Dao{
    pub const LEN = 8 + 128 + 32 + 2;
}
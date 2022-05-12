use anchor_lang::prelude::*;

pub type url = [u8;128];

#[account]
pub struct Issue{
    issue_url: url,                 // 128
    dao_pk: Pubkey,                 // 32
    issue_raiser_pubkey: Pubkey,    // 32
    issue_state: IssueState,        // 1
    frt_staked: u16,                // 2
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum IssueState{
    Open,
    Closed,
}

impl Issue{
    pub const LEN = 8 + 128 + 32 + 32 + 1 + 2;

    pub fn raise_issue(){

    }

    pub fn close_isssue(){

    }
}

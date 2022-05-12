use anchor_lang::prelude::*;

#[account]
pub struct Issue {
    issue_num: u16,              // 2
    dao_pk: Pubkey,              // 32
    issue_raiser_pubkey: Pubkey, // 32
    issue_state: IssueState,     // 1
    sol_staked: u64,             // 8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum IssueState {
    Open,
    Closed,
}

impl Issue {
    pub const LEN: usize = 8 + 2 + 32 + 32 + 1 + 8;

    pub const STAKETHRESHOLD: u64 = 1_000_000_000;

    pub fn raise_issue(
        &mut self,
        issue_num: u16,
        dao_pk: Pubkey,
        issue_raiser_pubkey: Pubkey,
        sol_staked: u64,
    ) {
        self.dao_pk = dao_pk;
        self.issue_raiser_pubkey = issue_raiser_pubkey;
        self.sol_staked = sol_staked;
        self.issue_state = IssueState::Open;
        self.issue_num = issue_num;
    }

    pub fn close_isssue() {}
}

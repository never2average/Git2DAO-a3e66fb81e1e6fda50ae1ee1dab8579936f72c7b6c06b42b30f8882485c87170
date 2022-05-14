use anchor_lang::prelude::*;

#[account]
pub struct Issue {
    issue_num: u16,              // 2
    dao_pk: Pubkey,              // 32
    issue_raiser_pubkey: Pubkey, // 32
    issue_state: IssueState,     // 1
    sol_staked: u64,             // 8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq,Copy)]
pub enum IssueState {
    NotFunded,
    Funded,
    Closed,
}

impl Issue {
    pub const LEN: usize = 8 + 2 + 32 + 32 + 1 + 8;

    pub const STAKETHRESHOLD: u64 = 10_000;

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
        self.issue_state = IssueState::Funded;
        self.issue_num = issue_num;
    }

    pub fn close_isssue() {}

    pub fn get_issue_state(&self) -> IssueState {
        self.issue_state
    }
}

use anchor_lang::prelude::*;

#[account]
pub struct Issue {
    pub issue_num: u16,              // 2
    pub dao: Pubkey,                 // 32
    pub issue_raiser: Pubkey,        // 32
    pub issue_state: IssueState,     // 1
    pub sol_staked: u64,             // 8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Copy)]
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
        dao: Pubkey,
        issue_raiser: Pubkey,
        sol_staked: u64,
    ) {
        self.dao = dao;
        self.issue_raiser = issue_raiser;
        self.sol_staked = sol_staked;
        self.issue_state = IssueState::Funded;
        self.issue_num = issue_num;
    }

    pub fn close_issue(&mut self) {
        self.issue_state = IssueState::Closed;
    }
}

use crate::error::DaoError;
use anchor_lang::prelude::*;

#[account]
pub struct Dao {
    pub repo_url: Vec<u8>,     // max 128
    pub owner: Pubkey, // 32
    pub funded_open_issue_count: u16,      // 2
    pub total_sol_staked: u64, // 8
}

impl Dao {
    pub const LEN: usize = 8 + 128 + 32 + 2 + 8;

    pub fn create_dao(&mut self, repo_url: String, owner: Pubkey) {
        self.repo_url = repo_url.into_bytes();
        self.owner = owner;
        self.funded_open_issue_count = 0;
        self.total_sol_staked = 0;
    }

    pub fn dao_update_issue_raised(&mut self, sol_staked: u64) -> Result<()> {
        let new_funded_open_issue_count = self.funded_open_issue_count.checked_add(1);

        require!(new_funded_open_issue_count.is_some(), DaoError::IssueOverflow);
        self.funded_open_issue_count = new_funded_open_issue_count.unwrap();
        self.stake_sol(sol_staked).unwrap();
        Ok(())
    }

    pub fn dao_update_issue_closed(&mut self, sol_staked: u64) -> Result<()> {
        self.funded_open_issue_count = self.funded_open_issue_count - 1;
        self.total_sol_staked -= sol_staked;
        Ok(())
    }

    fn stake_sol(&mut self, amount: u64) -> Result<()> {
        let new_total_sol_staked = self.total_sol_staked.checked_add(amount.into());

        require!(new_total_sol_staked.is_some(), DaoError::StakeOverflow);
        self.total_sol_staked = new_total_sol_staked.unwrap();
        Ok(())
    }
}

use crate::error::DaoError;
use anchor_lang::prelude::*;

#[account]
pub struct Dao {
    repo_url: Vec<u8>,         // max 128
    repo_owner_pk: Pubkey, // 32
    issue_count: u16,      // 2
    total_sol_staked: u64, // 8
    rent: u64,             // 8
}

impl Dao {
    pub const LEN: usize = 8 + 128 + 32 + 2 + 8 + 8;

    pub fn create_dao(&mut self, repo_url: String, repo_owner_pk: Pubkey) {
        self.repo_url = repo_url.into_bytes();
        self.repo_owner_pk = repo_owner_pk;
        self.issue_count = 0;
        self.total_sol_staked = 0;
        self.rent = Rent::default().minimum_balance(Dao::LEN);
    }

    pub fn dao_update_issue_raised(&mut self, sol_staked: u64) -> Result<()> {
        let new_issue_count = self.issue_count.checked_add(1);

        require!(new_issue_count.is_some(), DaoError::IssueOverflow);
        self.issue_count = new_issue_count.unwrap();

        self.stake_sol(sol_staked).unwrap();
        Ok(())
    }

    pub fn dao_update_issue_closed(&mut self, sol_staked: u64) -> Result<()> {
        self.issue_count = self.issue_count - 1;
        self.total_sol_staked -= sol_staked;
        Ok(())
    }

    fn stake_sol(&mut self, amount: u64) -> Result<()> {
        let new_total_sol_staked = self.total_sol_staked.checked_add(amount.into());

        require!(new_total_sol_staked.is_some(), DaoError::StakeOverflow);
        self.total_sol_staked = new_total_sol_staked.unwrap();
        Ok(())
    }

    pub fn get_issue_count(&self) -> u16 {
        self.issue_count
    }

    pub fn get_total_sol_staked(&self) -> u64 {
        self.total_sol_staked
    }

    pub fn get_rent_price(&self) -> u64 {
        self.rent
    }
}

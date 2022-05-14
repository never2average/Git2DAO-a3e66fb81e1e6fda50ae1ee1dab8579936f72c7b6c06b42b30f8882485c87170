use anchor_lang::prelude::*;

#[account]
pub struct User {
    owner_pk: Pubkey, // 32
    sol_balance: u64, // 8
}

impl User {
    pub const LEN: usize = 8 + 32 + 8;

    pub fn register(&mut self, owner_pk: Pubkey) {
        self.owner_pk = owner_pk;
        self.sol_balance = 0;   
    }

    pub fn get_owner_pk(&self) -> Pubkey {
        self.owner_pk
    }
}

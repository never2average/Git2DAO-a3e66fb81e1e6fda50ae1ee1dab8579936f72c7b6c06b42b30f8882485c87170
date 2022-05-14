use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,    // 32
    pub sol_balance: u64, // 8
}

impl User {
    pub const LEN: usize = 8 + 32 + 8;

    pub fn register(&mut self, owner_pk: Pubkey) {
        self.owner = owner_pk;
        self.sol_balance = 0;
    }
}

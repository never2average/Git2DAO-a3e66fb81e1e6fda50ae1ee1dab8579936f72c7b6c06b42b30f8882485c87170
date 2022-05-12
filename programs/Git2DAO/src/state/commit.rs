use anchor_lang::prelude::*;

#[account]
pub struct Commit{
    issue_pk: Pubkey,               // 32
    tree_hash: [u8;20]>             // 20 Byte Hash,
    parent_hash: [u8;20],           // 20
    commit_type: CommitType,        // 1
    commit_status: CommitStatus,    // 1
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum CommitType{
    Dummy,
    Actual,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum CommitStatus{
    Paid,
    NotPaid,
}

impl Commit{
    pub const LEN = 8 + 32 + 20 + 20 + 1 + 1;
}

      
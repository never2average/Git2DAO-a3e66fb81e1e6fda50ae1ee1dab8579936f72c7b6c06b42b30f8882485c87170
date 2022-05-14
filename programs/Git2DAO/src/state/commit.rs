use anchor_lang::prelude::*;

#[account]
pub struct Commit {
    pub issue: Pubkey,            // 32
    pub tree_hash: [u8; 20],         // 20 Byte Hash,
    pub commit_hash: [u8; 20],       // 20
    pub commit_type: CommitType,     // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum CommitType {
    Dummy,
    Actual,
}

impl Commit {
    pub const LEN: usize = 8 + 32 + 20 + 20 + 1 + 1;

    pub fn add_commit(&mut self,issue: Pubkey,tree_hash: [u8;20],commit_hash:[u8;20],commit_type: CommitType){
        self.issue = issue;
        self.tree_hash = tree_hash;
        self.commit_hash = commit_hash;
        self.commit_type = commit_type;
    }
}

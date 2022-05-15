use anchor_lang::error_code;

#[error_code]
pub enum DaoError {
    InvalidUrlLength,
    InvalidOwner,
    InvalidProgramOwner,
    IssueOverflow,
    StakeOverflow,
    ThresholdNotMet,
    InvalidPermission,
    InvalidCommitChain,
    NotCommitDataStructure,
    CommitAlreadyPaid,
    InvalidClaim
}

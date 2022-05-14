use anchor_lang::error_code;

#[error_code]
pub enum DaoError {
    InvalidUrlLength,
    InvalidOwner,
    IssueOverflow,
    StakeOverflow,
    ThresholdNotMet,
    IssueAlreadyRaised,
}
